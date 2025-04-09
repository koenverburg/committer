use crate::cmd::CommitCommandArgs;
use anyhow::Result;
use dialoguer::{Confirm, Editor, Input, MultiSelect, Select};
use std::process::Command as ProcessCommand;

#[derive(Debug, Copy, Clone)]
pub struct ScopeOption {
    pub name: &'static str,
    pub emoji: &'static str,
}

impl std::fmt::Display for ScopeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji, self.name)
    }
}

#[derive(Debug)]
pub struct CommitOptions {
    pub ticket_number: String,
    pub scope: ScopeOption,
    pub subject: String,
    pub message: String,
    pub description: String,
    pub tags: Vec<String>,
}

pub fn exec(args: CommitCommandArgs) -> Result<()> {
    // Handle preset if provided
    if let Some(preset) = &args.preset {
        match preset.as_str() {
            "formatting" => return exec_formatting_preset(args),
            "demo" => return exec_demo_preset(args),
            _ => {
                println!("Unknown preset: {}", preset);
                println!("Available presets: formatting, demo");
                return Ok(());
            }
        }
    }

    // Regular commit flow
    let options = collect_commit_info(args.ticket, args.no_description)?;
    let commit_message = format_commit_message(&options);

    println!("\nCommit message:\n{}", commit_message);

    let confirmation = Confirm::new()
        .with_prompt("Do you want to commit with this message?")
        .default(true)
        .interact()?;

    if confirmation {
        git_commit(&commit_message, args.no_verify)?;
        println!("Commit successful!");

        // Push if requested
        if args.push {
            println!("Pushing to remote...");
            git_push(args.force_push)?;
            println!("Push successful!");
        }
    } else {
        println!("Commit canceled.");
    }

    Ok(())
}

// Preset for formatting commits
fn exec_formatting_preset(args: CommitCommandArgs) -> Result<()> {
    // Use ticket from args or ask for it
    let ticket_number = if let Some(ticket) = args.ticket {
        ticket
    } else {
        Input::new()
            .with_prompt("Ticket number (optional)")
            .allow_empty(true)
            .interact()?
    };

    // Use fixed values for the rest
    let scope = ScopeOption {
        name: "style",
        emoji: "💎",
    };
    let subject = "formatting code";
    let message = "code formatting improvements";
    let description = "";
    let tags = Vec::new();

    let options = CommitOptions {
        ticket_number,
        scope,
        subject: subject.to_string(),
        message: message.to_string(),
        description: description.to_string(),
        tags,
    };

    let commit_message = format_commit_message(&options);

    println!("\nCommit message:\n{}", commit_message);

    let confirmation = Confirm::new()
        .with_prompt("Do you want to commit with this message?")
        .default(true)
        .interact()?;

    if confirmation {
        git_commit(&commit_message, args.no_verify)?;
        println!("Commit successful!");

        // Push if requested
        if args.push {
            println!("Pushing to remote...");
            git_push(args.force_push)?;
            println!("Push successful!");
        }
    } else {
        println!("Commit canceled.");
    }

    Ok(())
}

// Preset for demo commits
fn exec_demo_preset(args: CommitCommandArgs) -> Result<()> {
    // Use ticket from args or ask for it
    let ticket_number = if let Some(ticket) = args.ticket {
        ticket
    } else {
        Input::new()
            .with_prompt("Ticket number (optional)")
            .allow_empty(true)
            .interact()?
    };

    // Use fixed values for the rest
    let scope = ScopeOption {
        name: "demo",
        emoji: "🎮",
    };
    let subject = "demo changes";
    let message = "oh boi, I changed some files";
    let description = "";
    let tags = Vec::new();

    let options = CommitOptions {
        ticket_number,
        scope,
        subject: subject.to_string(),
        message: message.to_string(),
        description: description.to_string(),
        tags,
    };

    let commit_message = format_commit_message(&options);

    println!("\nCommit message:\n{}", commit_message);

    let confirmation = Confirm::new()
        .with_prompt("Do you want to commit with this message?")
        .default(true)
        .interact()?;

    if confirmation {
        git_commit(&commit_message, args.no_verify)?;
        println!("Commit successful!");

        // Push if requested
        if args.push {
            println!("Pushing to remote...");
            git_push(args.force_push)?;
            println!("Push successful!");
        }
    } else {
        println!("Commit canceled.");
    }

    Ok(())
}

fn collect_commit_info(
    ticket_override: Option<String>,
    no_description: bool,
) -> Result<CommitOptions> {
    println!("Please provide the following information for your commit:");

    // Available scope options with emojis
    let scope_options = [
        ScopeOption {
            name: "feat",
            emoji: "✨",
        }, // Sparkles
        ScopeOption {
            name: "fix",
            emoji: "🐛",
        }, // Bug
        ScopeOption {
            name: "docs",
            emoji: "📚",
        }, // Books
        ScopeOption {
            name: "style",
            emoji: "💎",
        }, // Gem
        ScopeOption {
            name: "refactor",
            emoji: "♻️",
        }, // Recycling
        ScopeOption {
            name: "perf",
            emoji: "🚀",
        }, // Rocket
        ScopeOption {
            name: "test",
            emoji: "🧪",
        }, // Test tube
        ScopeOption {
            name: "build",
            emoji: "🔨",
        }, // Hammer
        ScopeOption {
            name: "ci",
            emoji: "👷",
        }, // Construction worker
        ScopeOption {
            name: "chore",
            emoji: "🧹",
        }, // Broom
        ScopeOption {
            name: "revert",
            emoji: "⏪",
        }, // Rewind
        ScopeOption {
            name: "package",
            emoji: "📦",
        }, // Package
        ScopeOption {
            name: "draft",
            emoji: "📝",
        }, // Memo
        ScopeOption {
            name: "crash",
            emoji: "💥",
        }, // Explosion
        ScopeOption {
            name: "caution",
            emoji: "⚠️",
        }, // Warning
        ScopeOption {
            name: "danger",
            emoji: "🔥",
        }, // Fire
        ScopeOption {
            name: "hazard",
            emoji: "☢️",
        }, // Radioactive
        ScopeOption {
            name: "config",
            emoji: "⚙️",
        }, // Gear
        ScopeOption {
            name: "hack",
            emoji: "🔧",
        }, // Wrench
        ScopeOption {
            name: "bug",
            emoji: "🐞",
        }, // Lady Beetle
        ScopeOption {
            name: "fix",
            emoji: "🩹",
        }, // Adhesive Bandage
        ScopeOption {
            name: "wip",
            emoji: "🚧",
        }, // Construction
        ScopeOption {
            name: "trash",
            emoji: "🗑️",
        }, // Wastebasket
        ScopeOption {
            name: "deleting",
            emoji: "🧨",
        }, // Firecracker
        ScopeOption {
            name: "removal",
            emoji: "🔥",
        }, // Fire
    ];

    // Ticket number - use override if provided
    let ticket_number = if let Some(ticket) = ticket_override {
        ticket
    } else {
        Input::new()
            .with_prompt("Ticket number (optional)")
            .allow_empty(true)
            .interact()?
    };

    // Scope - using Select
    println!("Select the scope of your commit:");
    let scope_index = Select::new().items(&scope_options).default(0).interact()?;

    let scope = scope_options[scope_index];

    // Subject
    let subject: String = Input::new()
        .with_prompt("Subject (ex: payments/tests/delivery)")
        .interact()?;

    // Message
    let message: String = Input::new()
        .with_prompt("Message (main commit message)")
        .interact()?;

    // Description - skip if no_description flag is set
    let description = if no_description {
        String::new()
    } else {
        println!("Description (detailed explanation - an editor will open):");
        let editor_result = Editor::new()
            .edit("# Enter a detailed description of your changes\n# Lines starting with '#' will be ignored")?;

        // Remove comment lines from description
        editor_result
            .unwrap_or_default()
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<&str>>()
            .join("\n")
    };

    // Tags - first get available types
    let available_tags = vec!["[skip ci]", "(╯°□°)╯︵ ┻━┻"];

    println!("Select tags that apply to this commit:");
    let selections = MultiSelect::new().items(&available_tags).interact()?;

    let tags = selections
        .iter()
        .map(|&i| available_tags[i].to_string())
        .collect();

    Ok(CommitOptions {
        ticket_number,
        scope,
        subject,
        message,
        description,
        tags,
    })
}

pub fn format_commit_message(options: &CommitOptions) -> String {
    let ticket_prefix = if !options.ticket_number.is_empty() {
        format!("[{}] ", options.ticket_number)
    } else {
        String::new()
    };

    let tags_suffix = if !options.tags.is_empty() {
        format!(" - {}", options.tags.join(", "))
    } else {
        String::new()
    };

    let main_line = format!(
        "{}{} {}: {}{}",
        ticket_prefix, options.scope.emoji, options.scope.name, options.subject, tags_suffix
    );

    if options.description.trim().is_empty() {
        return format!("{} ({})", main_line, options.message);
    } else {
        return format!(
            "{} ({})\n\n{}",
            main_line, options.message, options.description
        );
    }
}

fn git_commit(message: &str, no_verify: bool) -> Result<()> {
    let mut cmd = ProcessCommand::new("git");
    cmd.arg("commit").arg("-m").arg(message);

    if no_verify {
        cmd.arg("--no-verify");
    }

    let status = cmd.status()?;

    if !status.success() {
        anyhow::bail!("Git commit command failed");
    }

    Ok(())
}

fn git_push(force: bool) -> Result<()> {
    let mut cmd = ProcessCommand::new("git");
    cmd.arg("push");

    if force {
        cmd.arg("--force");
    }

    let status = cmd.status()?;

    if !status.success() {
        anyhow::bail!("Git push command failed");
    }

    Ok(())
}
