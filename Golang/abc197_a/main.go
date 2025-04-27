package main

import (
	"bufio"
	"fmt"
	"os"
)

func read_line() string {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	text := scanner.Text()
	return text
}

func main() {
	text := read_line()

	result := solve(text)

	fmt.Println(result)
}

func solve(s string) string {

	x := s[:1]
	xs := s[1:]

	return xs + x
}
