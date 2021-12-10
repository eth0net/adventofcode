package main

import (
	_ "embed"
	"log"
	"strconv"
	"strings"

	_ "github.com/cmcpasserby/aoc"
)

//go:generate go run -mod=mod github.com/cmcpasserby/aoc/cmd/aoc --year 2021 --day 3 --output input.txt

//go:embed input.txt
var input string

func main() {
	lines := strings.Split(input, "\n")

	var counts []map[rune]int

	for _, line := range lines {
		for i, char := range line {
			for i >= len(counts) {
				counts = append(counts, map[rune]int{})
			}
			counts[i][char]++
		}
	}

	var gammaBin, epsilonBin string

	for _, count := range counts {
		switch {
		case count['0'] > count['1']:
			gammaBin += "0"
			epsilonBin += "1"
		case count['0'] < count['1']:
			gammaBin += "1"
			epsilonBin += "0"
		}
	}

	gamma, err := strconv.ParseInt(gammaBin, 2, 64)
	if err != nil {
		log.Fatalln("error parsing gamma rate:", err)
	}

	epsilon, err := strconv.ParseInt(epsilonBin, 2, 64)
	if err != nil {
		log.Fatalln("error parsing epsilon rate:", err)
	}

	log.Printf("Consumption: %d\n", gamma*epsilon)
}
