package internal

import (
	"fmt"
	"strings"

	"github.com/charmbracelet/huh"
)

var (
	ticket      string
	changeType  string
	scope       string
	subject     string
	tags        []string
	description string
)

type CommitMessage struct {
	Msg         string
	Description string
}

func RunForm() CommitMessage {
	form := huh.NewForm(
		huh.NewGroup(
			huh.NewInput().
				Title("Whats your ticket number?").
				Placeholder("XX-010101").
				Value(&ticket),

			huh.NewSelect[string]().
				Title("Choose your types").
				Options(
					huh.NewOption("feat", "✨feat"),
					huh.NewOption("fix", "🐛fix"),
					huh.NewOption("docs", "📚docs"),
					huh.NewOption("style", "💅style"),
					huh.NewOption("refactor", "🎨refactor"),
					huh.NewOption("chore", "🧹chore"),
					huh.NewOption("test", "🧪test"),
					huh.NewOption("hotfix", "🚑️hotfix"),
					huh.NewOption("deprecate", "⚰️deprecate"),
					huh.NewOption("perf", "⚡️perf"),
					huh.NewOption("wip", "🚧wip"),
					huh.NewOption("package", "📦package"),
				).
				Value(&changeType),

			huh.NewInput().
				Title("What the scope of the change").
				Value(&scope),

			huh.NewInput().
				Title("Summarize the changes").
				CharLimit(50).
				Placeholder("Oh boi I changes some files").
				Value(&subject),

			huh.NewMultiSelect[string]().
				Title("Tags").
				Options(
					huh.NewOption("wip", "WIP").Selected(false),
					huh.NewOption("skip ci", "[skip ci]").Selected(false),
					huh.NewOption("Table flip", "(╯°□°)╯︵ ┻━┻").Selected(false),
					huh.NewOption("Look of disapproval", "ಠ_ಠ").Selected(false),
				).
				Limit(2).
				Value(&tags),

			huh.NewInput().
				Title("Longer description of the changes").
				Value(&description),
		),
	)

	err := form.Run()
	CheckIfErrorFatal(err)

	message := fmt.Sprintf("%s %s(%s) %s %s", ticket, changeType, scope, subject, strings.Join(tags, " "))

	return CommitMessage{
		Msg:         strings.TrimSpace(message),
		Description: strings.TrimSpace(description),
	}
}
