package internal

import (
	"log"

	"github.com/sanity-io/litter"
)

func CheckIfErrorPanic(err error) {
	if err != nil {
		litter.D(err)
		panic(err)
	}
}

func CheckIfErrorFatal(err error) {
	if err != nil {
		litter.D(err)
		log.Fatal(err)
	}
}
