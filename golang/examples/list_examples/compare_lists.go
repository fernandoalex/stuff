// This example compare lists

// list 1
// ["item1","item2","item3"]

// list 2
// ["item1", "item2", "item4"]
package main

import (
"fmt"
)

// creating a function that
// receives: 2 arrays
// returns: the diff between
func getDiff(slice1 []string, slice2 []string) []string {

	var diff []string

	for i := 0; i < 2; i++ {
		for _, s1 := range(slice1) {
			found := false
			for _, s2 := range(slice2) {
				if s1 == s2 {
					found = true
					break
				}
			}
			if !found {
				diff = append(diff, s1)
			}
		}
		if i == 0 {
			slice1, slice2 = slice2, slice1
		}
	}


	return diff

}

func main() {

	list1 := make([]string, 3, 3)
	list1 = []string{"item1", "item2", "item3"}
	list2 := make([]string, 3,3)
	list2 = []string{"item1", "item2", "item4"}

	fmt.Println(getDiff(list1, list2))
}

