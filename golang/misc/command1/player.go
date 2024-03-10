package main

type Player struct {
	Name     string
	Position *Location
}

func NewPlayer(name string, location *Location) Player {
	return Player{
		Name:     name,
		Position: location,
	}

}
