package main

import (
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"

	_ "github.com/cmcpasserby/aoc"
)

//go:generate go run -mod=mod github.com/cmcpasserby/aoc/cmd/aoc --year 2021 --day 1 --output input.txt

//go:embed input.txt
var input string

func main() {
	var last, count int

	lines := strings.Fields(input)
	for i, line := range lines {
		depth, err := strconv.Atoi(line)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

		if depth > last && i > 0 {
			count++
		}

		last = depth
	}

	fmt.Printf("Depth increased %d times\n", count)
}
