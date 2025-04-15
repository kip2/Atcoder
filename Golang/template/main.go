package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	line := scanner.Text()
	fields := strings.Fields(line)

	// a, _ := strconv.Atoi(fields[0])
	// b, _ := strconv.Atoi(fields[1])
	// c, _ := strconv.Atoi(fields[2])

	result := solve()

	fmt.Println(result)
}

func solve() {
	// implement me!
}
