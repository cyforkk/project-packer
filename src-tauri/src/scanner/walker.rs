use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::rules::builtin::get_builtin_rules;
use crate::rules::matcher::{matches_any_rule, find_matching_rule};
use crate::types::CleanableItem;

/// 扫描项目目录，找出所有可清理的项
pub fn scan_cleanable_items(path: &Path) -> Vec<CleanableItem> {
    let rules = get_builtin_rules();
    let mut items = Vec::new();

    // 遍历项目根目录的直接子项
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return items,
    };

    for entry in entries.flatten() {
        let entry_path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if matches_any_rule(&name, &entry_path, &rules) {
            let size = calculate_size(&entry_path);
            let is_dir = entry_path.is_dir();
            let category = find_matching_rule(&name, &entry_path, &rules)
                .map(|r| r.category.clone())
                .unwrap_or(crate::types::CleanCategory::Temp);

            items.push(CleanableItem {
                path: entry_path.to_string_lossy().to_string(),
                name,
                size,
                category,
                is_dir,
            });
        }
    }

    // 递归查找匹配文件模式的文件（如 *.log）
    for walk_entry in WalkDir::new(path).min_depth(1).into_iter().flatten() {
        let walk_path = walk_entry.path();
        // 跳过已经作为目录项匹配的
        if walk_path.parent() == Some(path) && walk_path.is_dir() {
            continue;
        }
        let name = walk_entry.file_name().to_string_lossy().to_string();
        if matches_any_rule(&name, walk_path, &rules)
            && !items.iter().any(|i| i.path == walk_path.to_string_lossy().as_ref())
        {
            let category = find_matching_rule(&name, walk_path, &rules)
                .map(|r| r.category.clone())
                .unwrap_or(crate::types::CleanCategory::Temp);
            items.push(CleanableItem {
                path: walk_path.to_string_lossy().to_string(),
                name,
                size: calculate_size(walk_path),
                category,
                is_dir: false,
            });
        }
    }

    items
}

pub fn calculate_size(path: &Path) -> u64 {
    if path.is_file() {
        return fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    }
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

pub fn count_files(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_project() -> TempDir {
        let dir = TempDir::new().unwrap();
        // 创建 node_modules 目录（模拟依赖）
        let nm = dir.path().join("node_modules");
        fs::create_dir_all(&nm).unwrap();
        fs::write(nm.join("pkg.js"), "x".repeat(1000)).unwrap();

        // 创建 .git 目录
        let git = dir.path().join(".git");
        fs::create_dir_all(&git).unwrap();
        fs::write(git.join("config"), "gitconfig").unwrap();

        let prototypes = dir.path().join("prototypes");
        fs::create_dir_all(&prototypes).unwrap();
        fs::write(prototypes.join("demo.txt"), "prototype").unwrap();

        // 创建源代码文件
        fs::write(dir.path().join("index.js"), "console.log('hello')").unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();

        dir
    }

    #[test]
    fn scan_finds_node_modules() {
        let dir = create_test_project();
        let items = scan_cleanable_items(dir.path());
        assert!(items.iter().any(|i| i.name == "node_modules"));
    }

    #[test]
    fn scan_finds_git() {
        let dir = create_test_project();
        let items = scan_cleanable_items(dir.path());
        assert!(items.iter().any(|i| i.name == ".git"));
    }

    #[test]
    fn scan_finds_prototypes_directory() {
        let dir = create_test_project();
        let items = scan_cleanable_items(dir.path());
        assert!(items.iter().any(|i| {
            i.name == "prototypes"
                && i.is_dir
                && i.category == crate::types::CleanCategory::Dependency
        }));
    }

    #[test]
    fn scan_does_not_flag_source_files() {
        let dir = create_test_project();
        let items = scan_cleanable_items(dir.path());
        assert!(!items.iter().any(|i| i.name == "index.js"));
        assert!(!items.iter().any(|i| i.name == "package.json"));
    }

    #[test]
    fn calculate_size_of_directory() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("a.txt"), "hello").unwrap();
        fs::write(dir.path().join("b.txt"), "world!").unwrap();
        let size = calculate_size(dir.path());
        assert_eq!(size, 11); // 5 + 6
    }

    #[test]
    fn calculate_size_of_file() {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "12345").unwrap();
        let size = calculate_size(&file);
        assert_eq!(size, 5);
    }

    #[test]
    fn count_files_in_directory() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("a.txt"), "").unwrap();
        fs::write(dir.path().join("sub/b.txt"), "").unwrap();
        assert_eq!(count_files(dir.path()), 2);
    }
}
