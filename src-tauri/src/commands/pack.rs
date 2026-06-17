use std::path::Path;
use tauri::Emitter;
use crate::packer::{zipper, copier};
use crate::rules::builtin::{get_builtin_rules, categories_for_mode};
use crate::types::{PackConfig, PackResult, ProgressEvent, CleanMode};

#[tauri::command]
pub async fn pack_project(
    app: tauri::AppHandle,
    config: PackConfig,
) -> Result<PackResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let source = Path::new(&config.source_path);
        let output = Path::new(&config.output_path);

        if !source.exists() {
            return Err(format!("源路径不存在: {}", config.source_path));
        }

        // 防止输出路径在源目录内部
        let output_abs = if output.is_absolute() {
            output.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(output)
        };
        let source_abs = if source.is_absolute() {
            source.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(source)
        };
        if output_abs.starts_with(&source_abs) {
            return Err("输出路径不能保存在项目目录内，请选择其他位置".to_string());
        }

        // Determine excluded paths based on clean mode
        let excluded_paths = match config.clean_mode {
            CleanMode::Custom => config.excluded_paths.clone(),
            _ => {
                let rules = get_builtin_rules();
                let allowed_categories = categories_for_mode(&config.clean_mode);
                // For non-Custom modes, filter the cleanable items by category
                // The excluded_paths from frontend are the paths the user sees as cleanable
                config
                    .excluded_paths
                    .iter()
                    .filter(|path| {
                        let p = Path::new(path);
                        let name = p.file_name().unwrap_or_default().to_string_lossy();
                        rules.iter().any(|r| {
                            allowed_categories.contains(&r.category)
                                && r.matches(&name, p)
                        })
                    })
                    .cloned()
                    .collect()
            }
        };

        let app_clone = app.clone();
        let on_progress = move |event: ProgressEvent| {
            let _ = app_clone.emit("pack-progress", &event);
        };

        match config.output_type {
            crate::types::OutputType::Zip => {
                zipper::pack_to_zip(source, output, &excluded_paths, on_progress)
            }
            crate::types::OutputType::Copy => {
                copier::pack_to_directory(source, output, &excluded_paths, on_progress)
            }
        }
        .map_err(|e| format!("打包失败: {}", e))
    })
    .await
    .map_err(|e| format!("打包任务异常: {}", e))?
}
