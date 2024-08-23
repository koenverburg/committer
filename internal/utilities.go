package internal

import (
	"fmt"
	"log"
	"os/exec"
)

func CheckIfErrorPanic(err error) {
	if err != nil {
		panic(err)
	}
}

func CheckIfErrorFatal(err error) {
	if err != nil {
		log.Fatal(err)
	}
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
		fmt.Sprintf(`-m "%s"`, message),
		fmt.Sprintf(`-m "%s"`, description),
		"--quiet",
	)
	err = cmd.Run()
	CheckIfErrorFatal(err)
}
