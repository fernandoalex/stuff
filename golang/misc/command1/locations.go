package main

type Location struct {
	Name string
}

func NewLocation(name string) Location {
	return Location{
		Name: name,
	}
}
