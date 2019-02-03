package main

import (
    "fmt"
)

func main() {
    // A slice literal is declared just like an array literal, 
    //except you leave out the element count.
    // x := [5]int{2, 3, 4, 5, 76} // this is not a slice
    // x := []int{2, 3, 4, 5, 76} // this is a slice
    x := []int{2, 3, 4, 5, 76}
    fmt.Println(x)

    // for INDEX i, and VALUE v in range
    for i, v := range x {
        fmt.Println(i)
        fmt.Println(v)
    }
    // fmt.Println("%T", x) //this do not work, needs to be Printf (format printing)
    fmt.Printf("%T", x)
}

