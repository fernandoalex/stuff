package main

import (
	"fmt"
)

type person struct {
	first string
	last  string
	age   int
}

func (p person) speak() {
	fmt.Println("My name ", p.first, p.last, p.age)
}

func main() {
	p1 := person{
		first: "John",
		last:  "balba",
		age:   12,
	}
	p1.speak()

}
