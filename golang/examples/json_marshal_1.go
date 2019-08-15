package main

import (
	"fmt"
	"encoding/json"
	"os"
)

type user struct {
	First string
	Age   int
}

func main() {
	u1 := user{
		First: "James",
		Age:   32,
	}

	u2 := user{
		First: "Moneypenny",
		Age:   27,
	}

	u3 := user{
		First: "M",
		Age:   54,
	}

	users := []user{u1, u2, u3}

	fmt.Println(users)

	m, err := json.Marshal(users)

	if err != nil {
		fmt.Println(err)
	}
	// Marshal return a list of bytes, so we need to format it
	fmt.Fprintf(os.Stdout, "%s", m)
}
