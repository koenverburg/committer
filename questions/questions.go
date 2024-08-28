package questions

import (
	"strings"

	"github.com/charmbracelet/huh"
	"github.com/koenverburg/committer/helpers"
	"github.com/koenverburg/committer/internal"
)

// https://babakks.github.io/article/2020/07/03/emojis-in-git-commit-messages.html

var (
	ticket      string
	changeType  string
	scope       string
	subject     string
	tags        []string
	description string
)

type Commit struct {
	Msg         string
	Description string
}

func RunForm() Commit {
	form := huh.NewForm(
		huh.NewGroup(
			huh.NewInput().
				Title("Whats your ticket number?").
				Placeholder("XX-010101").
				Value(&ticket),

			huh.NewSelect[string]().
				Title("Choose your types").
				Options(
					huh.NewOption("🐞 bug", "🐞 bug"),
					huh.NewOption("🐛 bugfix", "🐛  bugfix"),
					huh.NewOption("🔨 improv", "🔨 improv"),
					huh.NewOption("🛠 refactor", "🛠 refactor"),
					huh.NewOption("🚧 wip", "🚧 wip"),
					huh.NewOption("🚫 hack", "🚫 hack"),
					huh.NewOption("⛔️ faulty", "⛔️ faulty"),
					huh.NewOption("📦 package", "📦 package"),
					huh.NewOption("🔥 trash", "🔥 trash"),
					huh.NewOption("💣 deleting", "💣 deleting"),
					huh.NewOption("🗑 removal", "🗑 removal"),
					huh.NewOption("🎨 style", "🎨 style"),
					huh.NewOption("💅 format", "💅 format"),
					huh.NewOption("✨ improve", "✨ improve"),
					huh.NewOption("👓 readability", "👓 readability"),
					huh.NewOption("🌱 feat", "🌱 feat"),
					huh.NewOption("🎉 feat", "🎉 feat"),
					huh.NewOption("🚿 clean", "🚿 clean"),
					huh.NewOption("🧪 test", "🧪 test"),
					huh.NewOption("🚀 deploy", "🚀 deploy"),
					huh.NewOption("📚 docs", "📚 docs"),
					huh.NewOption("📝 draft", "📝 draft"),
					huh.NewOption("⚠️ caution", "⚠️ caution"),
					huh.NewOption("💀 crash", "💀 crash"),
					huh.NewOption("☣️ danger", "☣️ danger"),
					huh.NewOption("☠️ hazard", "☠️ hazard"),
					huh.NewOption("⚙️ config", "⚙️ config"),
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
					huh.NewOption("wip", "#WIP").Selected(false),
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
	internal.CheckIfErrorFatal(err)

	// message := fmt.Sprintf("%s %s(%s) %s %s", ticket, changeType, scope, subject, strings.Join(tags, " "))
	message := helpers.CreateMessage(ticket, changeType, scope, subject, tags)

	return Commit{
		Msg:         strings.TrimSpace(message),
		Description: strings.TrimSpace(description),
	}
}
