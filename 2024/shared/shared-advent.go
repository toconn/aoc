package shared

import (
	"os"
	"strings"
)

func SelectForTestOrActual(actual string, test string) string {

	if IsActual() {
		return actual
	}

	return test
}

func IsActual() bool {
	return ! IsTest()
}

func IsTest() bool {

	arguments := os.Args[1:]

	if len(arguments) == 0 {
		return false
	}

	command := arguments[0]

	if strings.HasPrefix("actual", command) {
		return false
	}

	return strings.HasPrefix("test", command)
}