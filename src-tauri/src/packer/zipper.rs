use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;
use crate::types::{PackResult, ProgressEvent};

pub fn pack_to_zip(
    source: &Path,
    output: &Path,
    excluded_paths: &[String],
    mut on_progress: impl FnMut(ProgressEvent),
) -> std::io::Result<PackResult> {
    // 规范化输出路径，用于后续排除输出文件本身
    let output_abs = if output.is_absolute() {
        output.to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_default().join(output)
    };

    let total_files = count_packable_files(source, excluded_paths, &output_abs);
    let mut processed: u64 = 0;

    let file = fs::File::create(output)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let relative = entry_path.strip_prefix(source).unwrap_or(entry_path);

        if should_exclude(entry_path, source, excluded_paths) {
            continue;
        }

        // 跳过输出文件本身（防止 ZIP 被保存在源目录内时把自己打进去）
        let entry_abs = if entry_path.is_absolute() {
            entry_path.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(entry_path)
        };
        if entry_abs == output_abs {
            continue;
        }

        let relative_str = relative.to_string_lossy().replace('\\', "/");

        if entry.file_type().is_dir() {
            if !relative_str.is_empty() {
                zip.add_directory(&format!("{}/", relative_str), options)?;
            }
        } else {
            zip.start_file(&relative_str, options)?;
            let content = fs::read(entry_path)?;
            zip.write_all(&content)?;

            processed += 1;
            on_progress(ProgressEvent {
                percent: (processed as f64 / total_files as f64) * 100.0,
                current_file: relative_str.clone(),
            });
        }
    }

    zip.finish()?;

    let output_size = fs::metadata(output)?.len();
    Ok(PackResult {
        success: true,
        output_path: output.to_string_lossy().to_string(),
        total_files: processed,
        output_size,
        message: format!("打包完成，共 {} 个文件", processed),
    })
}

fn should_exclude(entry_path: &Path, _source: &Path, excluded_paths: &[String]) -> bool {
    excluded_paths.iter().any(|excluded| {
        let excluded_path = Path::new(excluded);
        entry_path == excluded_path || entry_path.starts_with(excluded_path)
    })
}

fn count_packable_files(source: &Path, excluded_paths: &[String], output_path: &Path) -> u64 {
    WalkDir::new(source)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| !should_exclude(e.path(), source, excluded_paths))
        .filter(|e| {
            let entry_abs = if e.path().is_absolute() {
                e.path().to_path_buf()
            } else {
                std::env::current_dir().unwrap_or_default().join(e.path())
            };
            entry_abs != output_path
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_project() -> TempDir {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("main.rs"), "fn main(){}").unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[package]").unwrap();
        let target = dir.path().join("target");
        fs::create_dir_all(target.join("debug")).unwrap();
        fs::write(target.join("debug/app"), "binary").unwrap();
        dir
    }

    #[test]
    fn zip_excludes_target_dir() {
        let source = create_test_project();
        let output_dir = TempDir::new().unwrap();
        let zip_path = output_dir.path().join("out.zip");
        let excluded = vec![source.path().join("target").to_string_lossy().to_string()];
        let result = pack_to_zip(source.path(), &zip_path, &excluded, |_| {}).unwrap();
        assert!(result.success);
        assert_eq!(result.total_files, 2);
    }

    #[test]
    fn zip_includes_all_when_no_exclusions() {
        let source = create_test_project();
        let output_dir = TempDir::new().unwrap();
        let zip_path = output_dir.path().join("out.zip");
        let result = pack_to_zip(source.path(), &zip_path, &[], |_| {}).unwrap();
        assert!(result.success);
        assert_eq!(result.total_files, 3);
    }

    #[test]
    fn zip_excludes_output_file_inside_source() {
        let source = create_test_project();
        // ZIP 文件保存在源目录内部
        let zip_path = source.path().join("output.zip");
        let result = pack_to_zip(source.path(), &zip_path, &[], |_| {}).unwrap();
        assert!(result.success);
        // 3 个源文件（main.rs + Cargo.toml + target/debug/app），不包含 output.zip
        assert_eq!(result.total_files, 3);
    }

    #[test]
    fn should_exclude_returns_true_for_excluded_path() {
        let excluded = vec!["/project/node_modules".to_string()];
        assert!(should_exclude(Path::new("/project/node_modules"), Path::new("/project"), &excluded));
        assert!(should_exclude(Path::new("/project/node_modules/pkg/index.js"), Path::new("/project"), &excluded));
        assert!(!should_exclude(Path::new("/project/src/main.js"), Path::new("/project"), &excluded));
    }
}
