package main
import (
    "fmt"
)
func main() {
    x := []int{2,3,4,5,76}
    fmt.Println(x)
    
    // for INDEX i, and VALUE v in range
    for i, v := range x {
        fmt.Println(i, v)
    }
}

