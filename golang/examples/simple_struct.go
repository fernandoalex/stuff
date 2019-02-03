package main

import "fmt"

type MyType1 struct {
	a int
}

func (t *MyType1) PlusOne() {
	t.a = t.a + 1
}

func main() {

	var1 := MyType1{1}
	var1.PlusOne()
	fmt.Print(var1.a)
}