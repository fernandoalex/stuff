package main

import (
	"os"
	"fmt"
)

func main() {
	var ENV_TEST string

	ENV_TEST = os.Getenv("ENV_TEST")
	fmt.Fprintf(os.Stdout, "My ENV is: %s", ENV_TEST)
}