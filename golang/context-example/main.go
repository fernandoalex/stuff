package main

import (
	"context"
	"fmt"
	"time"
)

func doSomething(ctx context.Context) {
	ctx, cancelCtx := context.WithCancel(ctx)
	printCh := make(chan int)

	go doAnother(ctx, printCh)
	for num := 1; num <= 3; num++{
		printCh <- num
	}

	cancelCtx()

	time.Sleep(100 * time.Millisecond)

	fmt.Printf("doSomething ended\n")
}

func doAnother(ctx context.Context, printCh <-chan int) {
	for {
		select {
		case <-ctx.Done():
			if err := ctx.Err(); err != nil {
				fmt.Printf("doAnother err: %s\n", err)
			}
			fmt.Printf("doAnother ended\n")
			return
		case num := <-printCh:
			fmt.Printf("doAnother: %d\n", num)
		
		}
	}
}

func main() {
	ctx := context.TODO()

	ctx = context.WithValue(ctx, "myKey","myValue")

	doSomething(ctx)
}
