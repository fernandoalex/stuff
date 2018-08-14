package main

import "fmt"

type oneInt struct {
	someValue int
}

func (x oneInt) plusOne() {
	y := x.someValue + 1
	fmt.Println(y)
}

func main() {
	foo := oneInt{1}
	foo.plusOne()
}