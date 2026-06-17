use std::path::Path;
use crate::types::CleanCategory;

/// 清理规则
#[derive(Debug, Clone)]
pub struct CleanRule {
    pub name: String,
    pub pattern: RulePattern,
    pub category: CleanCategory,
}

/// 规则匹配模式
#[derive(Debug, Clone)]
pub enum RulePattern {
    /// 精确匹配目录/文件名
    Exact(String),
    /// 匹配文件扩展名
    Extension(String),
    /// 匹配目录名（递归查找）
    DirName(String),
}

impl CleanRule {
    pub fn matches(&self, name: &str, path: &Path) -> bool {
        match &self.pattern {
            RulePattern::Exact(target) => name == target,
            RulePattern::Extension(ext) => {
                path.is_file() && name.ends_with(ext)
            }
            RulePattern::DirName(dir) => {
                path.is_dir() && name == dir
            }
        }
    }
}

/// 获取所有内置清理规则
pub fn get_builtin_rules() -> Vec<CleanRule> {
    vec![
        // === 依赖目录 ===
        rule("node_modules", RulePattern::Exact("node_modules".into()), CleanCategory::Dependency),
        rule("target", RulePattern::Exact("target".into()), CleanCategory::Dependency),
        rule("venv", RulePattern::Exact("venv".into()), CleanCategory::Dependency),
        rule(".venv", RulePattern::Exact(".venv".into()), CleanCategory::Dependency),
        rule("vendor", RulePattern::Exact("vendor".into()), CleanCategory::Dependency),
        rule(".m2", RulePattern::Exact(".m2".into()), CleanCategory::Dependency),
        rule("__pycache__", RulePattern::DirName("__pycache__".into()), CleanCategory::Dependency),
        rule(".eggs", RulePattern::Exact(".eggs".into()), CleanCategory::Dependency),

        // === 构建产物 ===
        rule("dist", RulePattern::Exact("dist".into()), CleanCategory::BuildArtifact),
        rule("build", RulePattern::Exact("build".into()), CleanCategory::BuildArtifact),
        rule("out", RulePattern::Exact("out".into()), CleanCategory::BuildArtifact),
        rule(".next", RulePattern::Exact(".next".into()), CleanCategory::BuildArtifact),
        rule(".nuxt", RulePattern::Exact(".nuxt".into()), CleanCategory::BuildArtifact),
        rule("prototypes", RulePattern::DirName("prototypes".into()), CleanCategory::Dependency),

        // === IDE 配置 ===
        rule(".idea", RulePattern::Exact(".idea".into()), CleanCategory::IdeConfig),
        rule(".vscode", RulePattern::Exact(".vscode".into()), CleanCategory::IdeConfig),
        rule(".settings", RulePattern::Exact(".settings".into()), CleanCategory::IdeConfig),
        rule(".project", RulePattern::Exact(".project".into()), CleanCategory::IdeConfig),
        rule(".classpath", RulePattern::Exact(".classpath".into()), CleanCategory::IdeConfig),

        // === 版本控制 ===
        rule(".git", RulePattern::Exact(".git".into()), CleanCategory::VCS),
        rule(".svn", RulePattern::Exact(".svn".into()), CleanCategory::VCS),

        // === 日志文件 ===
        rule("*.log", RulePattern::Extension(".log".into()), CleanCategory::Log),

        // === 临时文件 ===
        rule("*.tmp", RulePattern::Extension(".tmp".into()), CleanCategory::Temp),
        rule(".cache", RulePattern::Exact(".cache".into()), CleanCategory::Temp),
        rule(".DS_Store", RulePattern::Exact(".DS_Store".into()), CleanCategory::Temp),
        rule("Thumbs.db", RulePattern::Exact("Thumbs.db".into()), CleanCategory::Temp),

        // === 文档目录 ===
        rule("docs", RulePattern::Exact("docs".into()), CleanCategory::Docs),
    ]
}

fn rule(name: &str, pattern: RulePattern, category: CleanCategory) -> CleanRule {
    CleanRule {
        name: name.to_string(),
        pattern,
        category,
    }
}

/// 根据清理模式过滤规则，返回应该生效的分类列表
pub fn categories_for_mode(mode: &crate::types::CleanMode) -> Vec<CleanCategory> {
    use crate::types::CleanMode;
    match mode {
        CleanMode::Basic => vec![CleanCategory::Dependency],
        CleanMode::Medium => vec![
            CleanCategory::Dependency,
            CleanCategory::IdeConfig,
        ],
        CleanMode::Full => vec![
            CleanCategory::Dependency,
            CleanCategory::BuildArtifact,
            CleanCategory::IdeConfig,
            CleanCategory::VCS,
            CleanCategory::Log,
            CleanCategory::Temp,
        ],
        CleanMode::Custom => vec![], // Custom 模式由用户选择
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CleanMode;

    #[test]
    fn exact_rule_matches_directory() {
        let r = rule("node_modules", RulePattern::Exact("node_modules".into()), CleanCategory::Dependency);
        let path = Path::new("/project/node_modules");
        assert!(r.matches("node_modules", path));
        assert!(!r.matches("other", path));
    }

    #[test]
    fn extension_rule_matches_log_files() {
        let r = rule("*.log", RulePattern::Extension(".log".into()), CleanCategory::Log);
        let dir = tempfile::TempDir::new().unwrap();
        let log_file = dir.path().join("app.log");
        std::fs::write(&log_file, "log").unwrap();
        assert!(r.matches("app.log", &log_file));
        assert!(!r.matches("app.txt", &log_file));
    }

    #[test]
    fn builtin_rules_contain_node_modules() {
        let rules = get_builtin_rules();
        assert!(rules.iter().any(|r| r.name == "node_modules"));
    }

    #[test]
    fn builtin_rules_contain_git() {
        let rules = get_builtin_rules();
        assert!(rules.iter().any(|r| r.name == ".git"));
    }

    #[test]
    fn builtin_rules_contain_prototypes() {
        let rules = get_builtin_rules();
        assert!(rules.iter().any(|r| {
            r.name == "prototypes" && r.category == CleanCategory::Dependency
        }));
    }

    #[test]
    fn prototypes_rule_only_matches_directory() {
        let rules = get_builtin_rules();
        let rule = rules.iter().find(|r| r.name == "prototypes").unwrap();
        let dir_case = tempfile::TempDir::new().unwrap();
        let file_case = tempfile::TempDir::new().unwrap();
        let prototypes_dir = dir_case.path().join("prototypes");
        let prototypes_file = file_case.path().join("prototypes");
        std::fs::create_dir(&prototypes_dir).unwrap();
        std::fs::write(&prototypes_file, "not a directory").unwrap();
        assert!(rule.matches("prototypes", &prototypes_dir));
        assert!(!rule.matches("prototypes", &prototypes_file));
    }

    #[test]
    fn basic_mode_only_cleans_dependencies() {
        let cats = categories_for_mode(&CleanMode::Basic);
        assert_eq!(cats, vec![CleanCategory::Dependency]);
    }

    #[test]
    fn full_mode_cleans_everything() {
        let cats = categories_for_mode(&CleanMode::Full);
        assert_eq!(cats.len(), 6);
    }
}
