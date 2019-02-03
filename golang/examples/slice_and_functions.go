package main

import (
	"fmt"
	"encoding/json"
	"os"
)

func printSlice(newValue string, newSlice *[]string) {
	*newSlice = append(*newSlice, newValue) 
}

func main(){
	myList := make([]string, 0)
	printSlice("umacoisa", &myList)
	myList = append(myList, "uma coisa")
	b, _ := json.Marshal(myList)
	fmt.Fprintf(os.Stdout, "%s", b)
}