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

	cmd := exec.Command(fmt.Sprintf(`%s commit -m "%s" -m "%s" --quiet`, path, message, description))
	err = cmd.Run()
	CheckIfErrorFatal(err)
}
