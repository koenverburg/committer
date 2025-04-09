use anyhow::Result;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::commit::{format_commit_message, CommitOptions, ScopeOption};

    #[test]
    fn test_format_commit_message_with_ticket() -> Result<()> {
        let options = CommitOptions {
            ticket_number: "ABC-123".to_string(),
            scope: ScopeOption {
                name: "feat",
                emoji: "✨",
            },
            subject: "add user authentication".to_string(),
            message: "implement login system".to_string(),
            description: "Added OAuth2 login flow\nAdded session management".to_string(),
            tags: vec![],
        };

        let commit_message = format_commit_message(&options);
        assert!(commit_message.contains("[ABC-123]"));
        assert!(commit_message.contains("✨ feat"));
        assert!(commit_message.contains("add user authentication"));
        assert!(commit_message.contains("(implement login system)"));
        assert!(commit_message.contains("Added OAuth2 login flow"));

        Ok(())
    }

    #[test]
    fn test_format_commit_message_without_ticket() -> Result<()> {
        let options = CommitOptions {
            ticket_number: "".to_string(),
            scope: ScopeOption {
                name: "fix",
                emoji: "🐛",
            },
            subject: "broken login form".to_string(),
            message: "fix validation issues".to_string(),
            description: "Fixed validation on the login form".to_string(),
            tags: vec![],
        };

        let commit_message = format_commit_message(&options);
        assert!(!commit_message.contains("[]"));
        assert!(commit_message.contains("🐛 fix"));
        assert!(commit_message.contains("broken login form"));
        assert!(commit_message.contains("(fix validation issues)"));

        Ok(())
    }

    #[test]
    fn test_format_commit_message_with_tags() -> Result<()> {
        let options = CommitOptions {
            ticket_number: "DEF-456".to_string(),
            scope: ScopeOption {
                name: "docs",
                emoji: "📚",
            },
            subject: "update README".to_string(),
            message: "add installation instructions".to_string(),
            description: "Added detailed installation steps".to_string(),
            tags: vec!["[skip ci]".to_string()],
        };

        let commit_message = format_commit_message(&options);
        assert!(commit_message.contains("[DEF-456]"));
        assert!(commit_message.contains("📚 docs"));
        assert!(commit_message.contains("update README"));
        assert!(commit_message.contains("- [skip ci]"));

        Ok(())
    }

    #[test]
    fn test_format_commit_message_without_description() -> Result<()> {
        let options = CommitOptions {
            ticket_number: "XYZ-789".to_string(),
            scope: ScopeOption {
                name: "style",
                emoji: "💎",
            },
            subject: "format code".to_string(),
            message: "apply prettier formatting".to_string(),
            description: "".to_string(),
            tags: vec![],
        };

        let commit_message = format_commit_message(&options);
        assert!(commit_message.contains("[XYZ-789]"));
        assert!(commit_message.contains("💎 style"));
        assert!(commit_message.contains("format code"));
        assert!(commit_message.contains("(apply prettier formatting)"));
        assert!(!commit_message.contains("\n\n"));

        Ok(())
    }
}
