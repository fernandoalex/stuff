package main

import (
	"fmt"
	"encoding/json"
	"os"
)

func main(){
	myList := make([]string, 0)
	myList = append(myList, "uma coisa")
	b, _ := json.Marshal(myList)
	fmt.Fprintf(os.Stdout, "%s", b)
}