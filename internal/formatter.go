package internal

import (
	"fmt"
	"strings"
)

func FormatCommitMessage(ticket string, changeType string, scope string, subject string, tags []string) string {
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

	return strings.TrimSpace(fmt.Sprintln(strings.Join(messageSlice, " ")))
}

func FormatCommitString(message string, description string, noVerify bool) string {
	var slice []string

	slice = append(slice, "commit")
	slice = append(slice, fmt.Sprintf(`-m '%s'`, message))

	if len(strings.TrimSpace(description)) > 1 {
		slice = append(slice, fmt.Sprintf(`-m %s`, description))
	}

	slice = append(slice, "--quiet")

	if noVerify {
		slice = append(slice, "--no-verify")
	}

	joined := fmt.Sprintln(strings.Join(slice, ", "))
	return strings.TrimSpace(joined)
}

func FormatPushString(withForce bool) string {
	var slice []string

	slice = append(slice, "push")
	slice = append(slice, "--quiet")

	if withForce {
		slice = append(slice, "--force")
	}

	joined := fmt.Sprintln(strings.Join(slice, ", "))
	return strings.TrimSpace(joined)
}
