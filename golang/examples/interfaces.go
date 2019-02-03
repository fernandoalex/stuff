package main

import (
	"fmt"
)

type square1 struct {
	a int
}

func (s square1) area() int {
	return s.a * s.a
}

type square2 struct {
	a int
}

func (s square2) area() int {
	return s.a * s.a
}

type shape interface {
	area() int
}

func info(s shape) {
	fmt.Println(s.area())
}

func main() {
	s1 := square1{
		a: 2,
	}
	s2 := square2{
		a: 4,
	}
	info(s1)
	info(s2)
}
