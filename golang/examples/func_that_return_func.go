package main

import ("fmt")

func main() {
	var func1 = func() func() {
		return func() {
			fmt.Print("I am a returned func")
		}
	}
	a_func := func1()
	a_func()
}
