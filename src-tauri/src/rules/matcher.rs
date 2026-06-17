use std::path::Path;
use super::builtin::CleanRule;

/// 检查文件名/路径是否匹配任意规则
pub fn matches_any_rule(name: &str, path: &Path, rules: &[CleanRule]) -> bool {
    rules.iter().any(|r| r.matches(name, path))
}

/// 获取匹配到的规则
pub fn find_matching_rule<'a>(name: &str, path: &Path, rules: &'a [CleanRule]) -> Option<&'a CleanRule> {
    rules.iter().find(|r| r.matches(name, path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::builtin::get_builtin_rules;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn matches_node_modules() {
        let rules = get_builtin_rules();
        let dir = TempDir::new().unwrap();
        let nm = dir.path().join("node_modules");
        fs::create_dir(&nm).unwrap();
        assert!(matches_any_rule("node_modules", &nm, &rules));
    }

    #[test]
    fn does_not_match_source_file() {
        let rules = get_builtin_rules();
        let dir = TempDir::new().unwrap();
        let src = dir.path().join("main.rs");
        fs::write(&src, "fn main(){}").unwrap();
        assert!(!matches_any_rule("main.rs", &src, &rules));
    }

    #[test]
    fn find_matching_rule_returns_correct_rule() {
        let rules = get_builtin_rules();
        let dir = TempDir::new().unwrap();
        let git = dir.path().join(".git");
        fs::create_dir(&git).unwrap();
        let matched = find_matching_rule(".git", &git, &rules);
        assert!(matched.is_some());
        assert_eq!(matched.unwrap().name, ".git");
    }
}
