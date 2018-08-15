package main

import (
	"fmt"
	"encoding/json"
	"os"
)

func main(){
	myList := make([]string, 1)
	myList = append(myList, "uma coisa")
	b, _ := json.Marshal(myList)
	fmt.Fprintf(os.Stdout, "%s", b)
}