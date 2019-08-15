package main

import (
	"fmt"
)

func compareSlices(l1, l2 []string) []string {
/*
This function receives 2 slices of string and returns diff
*/
	var elementIsIn bool
	diff := make([]string, 1)

	for i := 0; i < len(l1); i++ {
		for y := 0; y < len(l2); y ++ {
			if l1[i] == l2[y] {
				elementIsIn = true
				break
			}
		}
		if !elementIsIn {
			diff = append(diff, l1[i])
		}
		elementIsIn = false
	}

	for i := 0; i < len(l2); i++ {
		for y := 0; y < len(l1); y ++ {
			if l2[i] == l1[y] {
				elementIsIn = true
				break
			}
		}
		if !elementIsIn {
			diff = append(diff, l2[i])
		}
		elementIsIn = false
	}

	return diff
}

func main() {
	list1 := make([]string, 2)
	list2 := make([]string, 2)
	list1 = []string{"potato", "oranges"}
	list2 = []string{"banana", "oranges"}

	fmt.Println(list1)
	fmt.Println(list2)
    fmt.Println(compareSlices(list1, list2))
}