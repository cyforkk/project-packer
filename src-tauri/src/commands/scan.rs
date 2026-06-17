use std::path::Path;
use crate::scanner::detector::detect_project_type;
use crate::scanner::walker::{scan_cleanable_items, calculate_size, count_files};
use crate::types::ProjectInfo;

#[tauri::command]
pub async fn scan_project(path: String) -> Result<ProjectInfo, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let p = Path::new(&path);
        if !p.exists() {
            return Err(format!("路径不存在: {}", path));
        }

        let project_type = detect_project_type(p);
        let cleanable_items = scan_cleanable_items(p);
        let total_size = calculate_size(p);
        let file_count = count_files(p);
        let cleanable_size: u64 = cleanable_items.iter().map(|i| i.size).sum();
        let estimated_clean_size = total_size.saturating_sub(cleanable_size);

        let name = p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string());

        Ok(ProjectInfo {
            path,
            name,
            project_type,
            total_size,
            estimated_clean_size,
            file_count,
            cleanable_items,
        })
    })
    .await
    .map_err(|e| format!("扫描任务异常: {}", e))?
}
