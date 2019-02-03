package main

import (
    "fmt"
)

func foo(xi ...int) {
    sum := 0
    for _, i := range xi {
        sum += i
    }
    fmt.Println(sum)
}

func bar(xi []int) {
    sum := 0
    for _, i := range xi {
        sum += i
    }
    fmt.Println(sum)
}

func main() {
    numbers1 := []int{1, 2, 3, 4, 5, 6}
    numbers2 := []int{1, 2, 3, 4}
    defer foo(numbers1...)
    bar(numbers2)

}

