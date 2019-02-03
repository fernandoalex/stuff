package main

import (
    "fmt"
)

func main() {
    states := make([]string, 2, 2)
    states = []string{"potato", "oranges"}
    fmt.Println(states)

    for i := 0; i < len(states); i++ {
        fmt.Println(i, states[i])
    }
}

