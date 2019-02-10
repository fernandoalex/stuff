package main

import ("fmt")

func main() {
	a_func := func (){
		for i := 0; i < 3; i++ {
			fmt.Println(i)
		}
	}

	a_func()
}