package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	// get input

	// output result value
	// result := solve()
	// fmt.Println(result)
}

func solve() {
	// implement me!
}

func read_line() string {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	text := scanner.Text()
	return text
}

func read_input_list(n int) []int {
	a := make([]int, n)

	for i := 0; i < n; i++ {
		fmt.Scan(&a[i])
	}

	return a
}
