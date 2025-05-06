package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	line := readIntList(2)
	n := line[0]
	x := line[1]
	nums := readIntList(n)

	result := solve(x, nums)
	resultStr := convIntArrToString(result)

	fmt.Println(resultStr)
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

func solve(x int, nums []int) []int {
	ret := make([]int, len(nums))
	cnt := 0
	for _, v := range nums {
		if v != x {
			ret[cnt] = v
			cnt++
		}
	}
	return ret[:cnt]
}

func readIntList(n int) []int {
	a := make([]int, n)

	for i := 0; i < n; i++ {
		fmt.Scan(&a[i])
	}

	return a
}
