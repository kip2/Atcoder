package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	// get input
	// s := read_line()
	// nums := read_input_list(5)

	// output result value
	// result := solve()
	// fmt.Println(result)
}

func solve() {
	// implement me!
}

func convIntArrToString(nums []int) string {
	strs := convIntArrToStringArr(nums)
	return strings.Join(strs, " ")
}

func convIntArrToStringArr(nums []int) []string {
	strs := make([]string, len(nums))
	for i, v := range nums {
		strs[i] = strconv.Itoa(v)
	}
	return strs
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
