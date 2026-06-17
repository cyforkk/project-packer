use serde::{Deserialize, Serialize};

/// 项目类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectType {
    Java,
    NodeJS,
    Python,
    Rust,
    Go,
    DotNet,
    PHP,
    Unknown,
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectType::Java => write!(f, "Java"),
            ProjectType::NodeJS => write!(f, "Node.js"),
            ProjectType::Python => write!(f, "Python"),
            ProjectType::Rust => write!(f, "Rust"),
            ProjectType::Go => write!(f, "Go"),
            ProjectType::DotNet => write!(f, ".NET"),
            ProjectType::PHP => write!(f, "PHP"),
            ProjectType::Unknown => write!(f, "未知"),
        }
    }
}

/// 清理分类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CleanCategory {
    Dependency,
    BuildArtifact,
    IdeConfig,
    VCS,
    Log,
    Temp,
    Docs,
}

impl std::fmt::Display for CleanCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CleanCategory::Dependency => write!(f, "依赖目录"),
            CleanCategory::BuildArtifact => write!(f, "构建产物"),
            CleanCategory::IdeConfig => write!(f, "IDE 配置"),
            CleanCategory::VCS => write!(f, "版本控制"),
            CleanCategory::Log => write!(f, "日志文件"),
            CleanCategory::Temp => write!(f, "临时文件"),
            CleanCategory::Docs => write!(f, "文档目录"),
        }
    }
}

/// 清理模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CleanMode {
    Basic,
    Medium,
    Full,
    Custom,
}

/// 输出类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputType {
    Zip,
    Copy,
}

/// 可清理项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanableItem {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub category: CleanCategory,
    pub is_dir: bool,
}

/// 项目信息（扫描结果）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub path: String,
    pub name: String,
    pub project_type: ProjectType,
    pub total_size: u64,
    pub estimated_clean_size: u64,
    pub file_count: u64,
    pub cleanable_items: Vec<CleanableItem>,
}

/// 打包配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackConfig {
    pub source_path: String,
    pub output_path: String,
    pub clean_mode: CleanMode,
    pub excluded_paths: Vec<String>,
    pub output_type: OutputType,
}

/// 打包结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackResult {
    pub success: bool,
    pub output_path: String,
    pub total_files: u64,
    pub output_size: u64,
    pub message: String,
}

/// 进度事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub percent: f64,
    pub current_file: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_type_display() {
        assert_eq!(ProjectType::Java.to_string(), "Java");
        assert_eq!(ProjectType::NodeJS.to_string(), "Node.js");
        assert_eq!(ProjectType::Unknown.to_string(), "未知");
    }

    #[test]
    fn test_clean_category_display() {
        assert_eq!(CleanCategory::Dependency.to_string(), "依赖目录");
        assert_eq!(CleanCategory::VCS.to_string(), "版本控制");
    }

    #[test]
    fn test_serialize_project_info() {
        let info = ProjectInfo {
            path: "/test".to_string(),
            name: "test".to_string(),
            project_type: ProjectType::Java,
            total_size: 1024,
            estimated_clean_size: 512,
            file_count: 10,
            cleanable_items: vec![],
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("Java"));
    }
}
