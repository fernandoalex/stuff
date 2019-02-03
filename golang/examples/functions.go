package main

import (
    "fmt"
)

func foo(xi ...int) int {
    sum := 0
    for _, i := range xi {
        sum += i
    }
    return sum
}

func bar(xi []int) int {
    sum := 0
    for _, i := range xi {
        sum += i
    }
    return sum
}

func main() {
    numbers := []int{1, 2, 3, 4}
    fmt.Println(foo(numbers...))
    fmt.Println(bar(numbers))

}

