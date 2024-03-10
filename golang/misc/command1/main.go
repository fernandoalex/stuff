package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	start()
}

func start() {

	fmt.Fprintf(os.Stdout, `
Welcome to the game, what you want to do?
(1) Start Game
(*) Exit

`)

	cli_command := getInput()
	if cli_command == "1" {
		gameLoop()
	}

}

func gameLoop() {

	gameRunning := true
	tavern := NewLocation("tavern")
	player := NewPlayer("player", &tavern)

	for gameRunning {

		fmt.Fprintf(os.Stdout, `
			you are at: %s

			what you want to do?
			(1) Go somewhere
			(0) Exit

			`, player.Position)

		if getInput() == "0" {
			gameRunning = false
		}

	}
}

func getInput() string {

	// prob move this logic to its own function to better manage error
	reader := bufio.NewReader(os.Stdin)
	buff_reader, err := reader.ReadString('\n')
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error while reading command input")
	}
	return strings.Trim(buff_reader, "\n")

}
