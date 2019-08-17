package main

import "testing"

func TestReturnsInt(t *testing.T) {
	got := ReturnsInt()
	if got != 5 {
		t.Errorf("got %d; want 5", got)
	}
}