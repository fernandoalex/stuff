package main

import "fmt"

func main() {
	s1 := make([]int, 2)
	s1[0] = 2
	s1[1] = 3
	fmt.Println(s1)
	s1 = append(s1, 5)
	fmt.Println(s1)
}
