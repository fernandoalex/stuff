package main

import "fmt"

func main() {
	i := 42

	fmt.Println(&i)
	p := &i         // point to i
	fmt.Println(&p)
	z := &p
	// y := *p //this does not work. Why?
	fmt.Println(*p)
	fmt.Println(*z)
	// fmt.Println(*y) // this does not work. Why?
}