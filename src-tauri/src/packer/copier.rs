use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::types::{PackResult, ProgressEvent};

pub fn pack_to_directory(
    source: &Path,
    output: &Path,
    excluded_paths: &[String],
    mut on_progress: impl FnMut(ProgressEvent),
) -> std::io::Result<PackResult> {
    if !output.exists() {
        fs::create_dir_all(output)?;
    }

    // 规范化输出路径，防止复制目标在源目录内时把输出目录自己复制进去
    let output_abs = if output.is_absolute() {
        output.to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_default().join(output)
    };

    let total_files = count_packable_files(source, excluded_paths, &output_abs);
    let mut processed: u64 = 0;

    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if should_exclude(entry_path, source, excluded_paths) {
            continue;
        }

        // 跳过输出目录本身及其子项
        let entry_abs = if entry_path.is_absolute() {
            entry_path.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(entry_path)
        };
        if entry_abs == output_abs || entry_abs.starts_with(&output_abs) {
            continue;
        }

        let relative = entry_path.strip_prefix(source).unwrap_or(entry_path);
        let dest_path = output.join(relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry_path, &dest_path)?;

            processed += 1;
            on_progress(ProgressEvent {
                percent: (processed as f64 / total_files as f64) * 100.0,
                current_file: relative.to_string_lossy().to_string(),
            });
        }
    }

    let output_size = calculate_dir_size(output);
    Ok(PackResult {
        success: true,
        output_path: output.to_string_lossy().to_string(),
        total_files: processed,
        output_size,
        message: format!("复制完成，共 {} 个文件", processed),
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
            entry_abs != output_path && !entry_abs.starts_with(output_path)
        })
        .count() as u64
}

fn calculate_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_project() -> TempDir {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("app.py"), "print('hello')").unwrap();
        fs::write(dir.path().join("requirements.txt"), "flask").unwrap();
        let venv = dir.path().join("venv");
        fs::create_dir_all(venv.join("lib")).unwrap();
        fs::write(venv.join("lib/pkg.pyc"), "bytecode").unwrap();
        dir
    }

    #[test]
    fn copy_excludes_venv() {
        let source = create_test_project();
        let output = TempDir::new().unwrap();
        let dest = output.path().join("clean_project");
        let excluded = vec![source.path().join("venv").to_string_lossy().to_string()];
        let result = pack_to_directory(source.path(), &dest, &excluded, |_| {}).unwrap();
        assert!(result.success);
        assert_eq!(result.total_files, 2);
        assert!(dest.join("app.py").exists());
        assert!(!dest.join("venv").exists());
    }

    #[test]
    fn copy_creates_output_directory() {
        let source = create_test_project();
        let output = TempDir::new().unwrap();
        let dest = output.path().join("new_dir");
        pack_to_directory(source.path(), &dest, &[], |_| {}).unwrap();
        assert!(dest.exists());
    }
}
