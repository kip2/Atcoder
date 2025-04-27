package main

import (
	"fmt"
)

func read_input_list(n int) []int {
	a := make([]int, n)

	for i := 0; i < n; i++ {
		fmt.Scan(&a[i])
	}

	return a
}

func main() {
	var n int
	fmt.Scan(&n)

	a_list := read_input_list(n)
	b_list := read_input_list(n)

	result := solve(a_list, b_list)

	fmt.Println(result)
}

func solve(a_list []int, b_list []int) string {

	sum := 0

	for i := 0; i < len(a_list); i++ {
		sum += a_list[i] * b_list[i]
	}

	if sum == 0 {
		return "Yes"
	}
	return "No"
}
