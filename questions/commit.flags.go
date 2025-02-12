package questions

import (
	"github.com/charmbracelet/huh"
	"github.com/koenverburg/committer/internal"
)

type CommitFlags struct {
	NoVerify bool
}

var (
	noVerify bool
)

func StartCommitFlagForm() CommitFlags {
	form := huh.NewForm(
		huh.NewGroup(
			huh.NewConfirm().
				Title("Add --no-verify flag").
				Value(&noVerify),
		),
	)

	err := form.Run()
	internal.CheckIfErrorFatal(err)

	return CommitFlags{
		NoVerify: noVerify,
	}
}
