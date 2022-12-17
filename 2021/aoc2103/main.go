package main

import (
	_ "embed"
	"fmt"
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

	power, err := parsePowerConsumption(lines)
	if err != nil {
		log.Fatalln("failed to parse power consumption:", err)
	}

	life, err := parseLifeSupportRating(lines)
	if err != nil {
		log.Fatalln("failed to parse power consumption:", err)
	}

	log.Printf("Power Consumption: %d\n", power)
	log.Printf("Life Support Rating: %d\n", life)
}

func countCharacters(lines []string) (counts []map[rune]int) {
	for _, line := range lines {
		for i, char := range line {
			for i >= len(counts) {
				counts = append(counts, map[rune]int{})
			}
			counts[i][char]++
		}
	}

	return counts
}

func parsePowerConsumption(lines []string) (power int, err error) {
	counts := countCharacters(lines)

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
		return 0, fmt.Errorf("error parsing gamma rate: %w", err)
	}

	epsilon, err := strconv.ParseInt(epsilonBin, 2, 64)
	if err != nil {
		return 0, fmt.Errorf("error parsing epsilon rate: %w", err)
	}

	return int(gamma * epsilon), nil
}

func parseLifeSupportRating(lines []string) (life int, err error) {
	oxygen, err := parseOxygenRating(lines)
	if err != nil {
		return 0, fmt.Errorf("error parsing oxygen rating")
	}

	carbon, err := parseCO2Rating(lines)
	if err != nil {
		return 0, fmt.Errorf("error parsing co2 rating")
	}

	return oxygen * carbon, nil
}

func parseOxygenRating(lines []string) (int, error) {
	for i := 0; len(lines) > 1; i++ {
		count := countCharacters(lines)[i]

		var c uint8 = '1'
		if count['0'] > count['1'] {
			c = '0'
		}

		lines = filterStrings(lines, func(s string) bool {
			if len(s) <= i || s[i] != c {
				return false
			}
			return true
		})

		if len(lines) == 1 {
			break
		}
	}

	oxygen, err := strconv.ParseInt(lines[0], 2, 64)
	if err != nil {
		return 0, fmt.Errorf("error parsing binary value: %w", err)
	}

	return int(oxygen), nil
}

func parseCO2Rating(lines []string) (int, error) {
	for i := 0; len(lines) > 1; i++ {
		count := countCharacters(lines)[i]

		var c uint8 = '0'
		if count['1'] < count['0'] {
			c = '1'
		}

		lines = filterStrings(lines, func(s string) bool {
			if len(s) <= i || s[i] != c {
				return false
			}
			return true
		})
	}

	carbon, err := strconv.ParseInt(lines[0], 2, 64)
	if err != nil {
		return 0, fmt.Errorf("error parsing binary value: %w", err)
	}

	return int(carbon), nil
}

func filterStrings(in []string, fn func(string) bool) (out []string) {
	for _, v := range in {
		if fn(v) {
			out = append(out, v)
		}
	}
	return out
}
