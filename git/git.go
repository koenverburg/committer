package git

import (
	"fmt"
	"log"
	"os/exec"
	"strings"

	"github.com/sanity-io/litter"
)

func getGitPath() string {
	path, err := exec.LookPath("git")

	if err != nil {
		panic("Git not found in PATH\n")
	}

	return path
}

func handleError(output []byte, err error) error {
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			return fmt.Errorf("command failed with exit code %d: %s",
				exitError.ExitCode(), string(output))
		}
	}

	return nil
}

func Commit(cmd string) error {
	log.SetFlags(log.LstdFlags | log.Lshortfile)

	git := getGitPath()
	parts := strings.Split(cmd, ", ")

	litter.D(parts)

	process := exec.Command(
		git,
		parts...,
	)

	log.Printf("commit cmd is: %s", process.String())

	output, err := process.CombinedOutput()
	return handleError(output, err)
}

func Push(cmd string) error {
	log.SetFlags(log.LstdFlags | log.Lshortfile)

	git := getGitPath()
	parts := strings.Split(cmd, ", ")

	process := exec.Command(
		git,
		parts...,
	)

	log.Printf("push cmd is: %s", process.String())

	output, err := process.CombinedOutput()
	return handleError(output, err)
}
