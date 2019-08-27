// Cli-args is a echo clone created for testing
package book_the_go_prog_lang

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	fmt.Println(strings.Join(os.Args[1:], " "))
}