package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	line := scanner.Text()
	fields := strings.Fields(line)

	a, _ := strconv.Atoi(fields[0])
	b, _ := strconv.Atoi(fields[1])
	c, _ := strconv.Atoi(fields[2])

	result := solve(a, b, c)

	fmt.Println(result)
}

func solve(a int, b int, c int) string {
	if a*a+b*b < c*c {
		return "Yes"
	}

	return "No"
}
