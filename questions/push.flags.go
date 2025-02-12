package questions

import (
	"github.com/charmbracelet/huh"
	"github.com/koenverburg/committer/internal"
)

type Push struct {
	Push          bool
	PushWithForce bool
}

var (
	push          bool
	pushWithForce bool
)

func StartPushForm() Push {
	form := huh.NewForm(
		huh.NewGroup(
			huh.NewConfirm().
				Title("Push").
				Value(&push),
			huh.NewConfirm().
				Title("Do we need to force it?").
				Value(&pushWithForce),
		),
	)

	err := form.Run()
	internal.CheckIfErrorFatal(err)

	return Push{
		Push:          push,
		PushWithForce: pushWithForce,
	}
}
