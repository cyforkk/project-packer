use std::fs;
use std::path::Path;
use crate::types::ProjectType;

pub fn detect_project_type(path: &Path) -> ProjectType {
    let markers = [
        ("pom.xml", ProjectType::Java),
        ("build.gradle", ProjectType::Java),
        ("build.gradle.kts", ProjectType::Java),
        ("package.json", ProjectType::NodeJS),
        ("Cargo.toml", ProjectType::Rust),
        ("go.mod", ProjectType::Go),
        ("composer.json", ProjectType::PHP),
        ("requirements.txt", ProjectType::Python),
        ("Pipfile", ProjectType::Python),
        ("setup.py", ProjectType::Python),
        ("pyproject.toml", ProjectType::Python),
    ];

    for (file, project_type) in markers {
        if path.join(file).exists() {
            return project_type;
        }
    }

    // 检查 .sln / .csproj 文件
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".sln") || name_str.ends_with(".csproj") {
                return ProjectType::DotNet;
            }
        }
    }

    ProjectType::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn detect_java_maven() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("pom.xml"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Java);
    }

    #[test]
    fn detect_java_gradle() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("build.gradle"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Java);
    }

    #[test]
    fn detect_nodejs() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::NodeJS);
    }

    #[test]
    fn detect_python_requirements() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("requirements.txt"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Python);
    }

    #[test]
    fn detect_python_pipfile() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("Pipfile"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Python);
    }

    #[test]
    fn detect_rust() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Rust);
    }

    #[test]
    fn detect_go() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("go.mod"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Go);
    }

    #[test]
    fn detect_dotnet() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("App.sln"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::DotNet);
    }

    #[test]
    fn detect_php() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("composer.json"), "{}").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::PHP);
    }

    #[test]
    fn detect_unknown() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("readme.txt"), "hello").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Unknown);
    }
}
