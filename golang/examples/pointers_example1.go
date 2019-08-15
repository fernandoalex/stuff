package main

import "fmt"

type person struct {
	name string
}

func changeMe (p *person) {
	p1 := p
	p1.name = "potatoes"
}

func main() {
	p2 := person{
		name: "orange",
	}

	fmt.Println(p2.name)
	changeMe(&p2)
	fmt.Println(p2.name)

}
