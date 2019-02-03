package main

import "fmt"

func functionA(x int) {
	fmt.Println(x)
}

//// this is a method
//func (x int) functionB() {
//	fmt.Println(x)
//}

func main() {
	functionA(1)
//	functionB(1)
}
