package main

import (
	_ "embed"
	"fmt"
	"log"
	"strconv"
	"strings"

	_ "github.com/cmcpasserby/aoc"
)

//go:generate go run -mod=mod github.com/cmcpasserby/aoc/cmd/aoc --year 2021 --day 1 --output input.txt

//go:embed input.txt
var input string

func main() {
	lines := strings.Fields(input)

	count, err := countIncLines(lines)
	if err != nil {
		log.Fatalln("failed to count lines:", err)
	}

	log.Printf("Line depth increased %d times\n", count)

	count, err = countIncWindows(lines)
	if err != nil {
		log.Fatalln("failed to count windows:", err)
	}

	log.Printf("Window depth increased %d times\n", count)
}

func countIncLines(lines []string) (count int, err error) {
	var last int

	for i, line := range lines {
		var depth int

		depth, err = strconv.Atoi(line)
		if err != nil {
			return 0, fmt.Errorf("atoi failed: %w", err)
		}

		if depth > last && i > 0 {
			count++
		}

		last = depth
	}

	return count, nil
}

func countIncWindows(lines []string) (count int, err error) {
	windows := make([]int, len(lines))

	for i, line := range lines {
		var depth int

		depth, err = strconv.Atoi(line)
		if err != nil {
			return 0, fmt.Errorf("atoi failed: %w", err)
		}

		for w := i; w > i-3 && w >= 0; w-- {
			windows[w] += depth
		}
	}

	var last int
	for i, window := range windows {
		if window > last && i > 0 {
			count++
		}

		last = window
	}

	return count, nil
}
