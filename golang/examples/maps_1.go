package main

import "fmt"

func main()  {
	m := make(map[int]int)

	m[0] = 22
	m[0]++
	fmt.Println(m[0])
	fmt.Println(m[1])
}
