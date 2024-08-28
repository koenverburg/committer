package helpers

import (
	"fmt"
	"os/exec"
	"strings"

	internal "github.com/koenverburg/committer/internal"
)

func CreateMessage(ticket string, changeType string, scope string, subject string, tags []string) string {
	var messageSlice []string

	if ticket != "" {
		messageSlice = append(messageSlice, ticket)
	}

	if scope != "" && changeType != "" {
		messageSlice = append(messageSlice, fmt.Sprintf("%s(%s)", changeType, scope))
	} else if scope == "" && changeType != "" {
		messageSlice = append(messageSlice, fmt.Sprintf("%s:", changeType))
	}

	if subject != "" {
		messageSlice = append(messageSlice, subject)
	}

	for _, tag := range tags {
		if tag != "" {
			messageSlice = append(messageSlice, tag)
		}
	}

	return fmt.Sprintln(strings.Join(messageSlice, " "))
}

func Commit(message string, description string) {
	path, err := exec.LookPath("git")

	if err != nil {
		fmt.Printf("Git not found in PATH\n")
		return
	}

	cmd := exec.Command(
		path,
		"commit",
		fmt.Sprintf(`-m %s`, message),
		fmt.Sprintf(`-m %s`, description),
		"--quiet",
	)
	err = cmd.Run()
	internal.CheckIfErrorFatal(err)
}
