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

        if args.push & !args.push_no_verify {
            println!("Pushing to remote...");
            git_push(args.force_push, args.push_no_verify)?;
            println!("Push successful!");
        }

        if args.push & args.push_no_verify {
            println!("Pushing without verify to remote...");
            git_push(args.force_push, args.push_no_verify)?;
            println!("Push successful!");
        }

    } else {
        println!("Commit canceled.");
    }

    Ok(())
}

fn exec_formatting_preset(args: CommitCommandArgs) -> Result<()> {
    let ticket_number = if let Some(ticket) = args.ticket {
        ticket
    } else {
        Input::new()
            .with_prompt("Ticket number (optional)")
            .allow_empty(true)
            .interact()?
    };

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

        if args.push {
            println!("Pushing to remote...");
            git_push(args.force_push, args.push_no_verify)?;
            println!("Push successful!");
        }
    } else {
        println!("Commit canceled.");
    }

    Ok(())
}

fn exec_demo_preset(args: CommitCommandArgs) -> Result<()> {
    let ticket_number = if let Some(ticket) = args.ticket {
        ticket
    } else {
        Input::new()
            .with_prompt("Ticket number (optional)")
            .allow_empty(true)
            .interact()?
    };

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

        if args.push {
            println!("Pushing to remote...");
            git_push(args.force_push, args.push_no_verify)?;
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

    let scope_options = [
        ScopeOption {
            name: "feat",
            emoji: "✨",
        },
        ScopeOption {
            name: "fix",
            emoji: "🐛",
        },
        ScopeOption {
            name: "docs",
            emoji: "📚",
        },
        ScopeOption {
            name: "style",
            emoji: "💎",
        },
        ScopeOption {
            name: "refactor",
            emoji: "♻️",
        },
        ScopeOption {
            name: "perf",
            emoji: "🚀",
        },
        ScopeOption {
            name: "test",
            emoji: "🧪",
        },
        ScopeOption {
            name: "build",
            emoji: "🔨",
        },
        ScopeOption {
            name: "ci",
            emoji: "👷",
        },
        ScopeOption {
            name: "chore",
            emoji: "🧹",
        },
        ScopeOption {
            name: "revert",
            emoji: "⏪",
        },
        ScopeOption {
            name: "package",
            emoji: "📦",
        },
        ScopeOption {
            name: "draft",
            emoji: "📝",
        },
        ScopeOption {
            name: "crash",
            emoji: "💥",
        },
        ScopeOption {
            name: "caution",
            emoji: "⚠️",
        },
        ScopeOption {
            name: "danger",
            emoji: "🔥",
        },
        ScopeOption {
            name: "hazard",
            emoji: "☢️",
        },
        ScopeOption {
            name: "config",
            emoji: "⚙️",
        },
        ScopeOption {
            name: "hack",
            emoji: "🔧",
        },
        ScopeOption {
            name: "bug",
            emoji: "🐞",
        },
        ScopeOption {
            name: "fix",
            emoji: "🩹",
        },
        ScopeOption {
            name: "wip",
            emoji: "🚧",
        },
        ScopeOption {
            name: "trash",
            emoji: "🗑️",
        },
        ScopeOption {
            name: "deleting",
            emoji: "🧨",
        },
        ScopeOption {
            name: "removal",
            emoji: "🔥",
        },
    ];

    let ticket_number = if let Some(ticket) = ticket_override {
        ticket
    } else {
        Input::new()
            .with_prompt("Ticket number (optional)")
            .allow_empty(true)
            .interact()?
    };

    println!("Select the scope of your commit:");
    let scope_index = Select::new().items(&scope_options).default(0).interact()?;

    let scope = scope_options[scope_index];

    let subject: String = Input::new()
        .with_prompt("Subject (ex: payments/tests/delivery)")
        .allow_empty(true)
        .interact()?;

    let message: String = Input::new()
        .with_prompt("Message (main commit message)")
        .interact()?;

    let description = if no_description {
        String::new()
    } else {
        println!("Description (detailed explanation - an editor will open):");
        let editor_result = Editor::new()
            .edit("# Enter a detailed description of your changes\n# Lines starting with '#' will be ignored")?;

        editor_result
            .unwrap_or_default()
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<&str>>()
            .join("\n")
    };

    let available_tags = vec![
        "#skip-ci",
        "#start-ci",
        "#start-deploy"
    ];

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
        format!("{} ", options.ticket_number)
    } else {
        String::new()
    };

    let tags_suffix = if !options.tags.is_empty() {
        format!("{}", options.tags.join(", "))
    } else {
        String::new()
    };

    let subject_formatted = if !options.subject.is_empty() {
        format!("({}):", options.subject)
    } else {
        ":".to_string()
    };

    let main_line = format!(
        "{}{}{}{}",
        ticket_prefix, options.scope.emoji, options.scope.name, subject_formatted
    );

    if options.description.trim().is_empty() {
        format!("{} {}{}", main_line, options.message, tags_suffix)
    } else {
        format!(
            "{} {} {}\n\n{}",
            main_line, options.message, tags_suffix, options.description
        )
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

fn git_push(force: bool, no_verify: bool) -> Result<()> {
    let mut cmd = ProcessCommand::new("git");
    cmd.arg("push");

    if force {
        cmd.arg("--force");
    }

    if no_verify {
        cmd.arg("--no-verify");
    }

    let status = cmd.status()?;

    if !status.success() {
        anyhow::bail!("Git push command failed");
    }

    Ok(())
}
