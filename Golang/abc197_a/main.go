package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	text := scanner.Text()

	result := solve(text)

	fmt.Println(result)
}

func solve(s string) string {

	x := s[:1]
	xs := s[1:]

	return xs + x
}
