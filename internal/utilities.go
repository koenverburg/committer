package internal

import (
	"log"
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
