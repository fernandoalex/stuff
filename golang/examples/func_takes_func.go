package main

import (
	"fmt"
)

func foo(f func()) {
	func1 := f
	func1()
}

func bar() {
	fmt.Println("I am a func")
}
func main(){
	foo(bar)
}