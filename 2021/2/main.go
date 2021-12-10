package main

import (
	_ "embed"
	"fmt"
	"log"
	"strconv"
	"strings"

	_ "github.com/cmcpasserby/aoc"
)

//go:generate go run -mod=mod github.com/cmcpasserby/aoc/cmd/aoc --year 2021 --day 2 --output input.txt

//go:embed input.txt
var input string

func main() {
	lines := strings.Split(input, "\n")

	var (
		sub1 submarine
		sub2 submarine2
	)

	for _, line := range lines {
		if line == "" {
			continue
		}

		if err := sub1.command(line); err != nil {
			log.Fatalln("sub1: failed to run command:", err)
		}

		if err := sub2.command(line); err != nil {
			log.Fatalln("sub2: failed to run command:", err)
		}
	}

	log.Printf("Sub1 position: %d\n", sub1.horizontal*sub1.vertical)
	log.Printf("Sub2 position: %d\n", sub2.horizontal*sub2.vertical)
}

type submarine struct {
	horizontal, vertical int
}

func (s *submarine) command(commands ...string) error {
	for _, cmd := range commands {
		direction, distance, err := parseCommand(cmd)
		if err != nil {
			return fmt.Errorf("error parsing command: %w", err)
		}

		switch direction {
		case directionUp:
			s.vertical -= distance
		case directionDown:
			s.vertical += distance
		case directionBack:
			s.horizontal -= distance
		case directionForward:
			s.horizontal += distance
		}
	}

	return nil
}

type submarine2 struct {
	horizontal, vertical, aim int
}

func (s *submarine2) command(commands ...string) error {
	for _, cmd := range commands {
		direction, distance, err := parseCommand(cmd)
		if err != nil {
			return fmt.Errorf("error parsing command: %w", err)
		}

		switch direction {
		case directionUp:
			s.aim -= distance
		case directionDown:
			s.aim += distance
		case directionBack:
			s.horizontal -= distance
			s.vertical -= distance * s.aim
		case directionForward:
			s.horizontal += distance
			s.vertical += distance * s.aim
		}
	}

	return nil
}

func parseCommand(cmd string) (direction string, distance int, err error) {
	f := strings.Fields(cmd)

	direction = f[0]

	distance, err = strconv.Atoi(f[1])
	if err != nil {
		return "", 0, err
	}

	return direction, distance, nil
}

const (
	directionUp      = "up"
	directionDown    = "down"
	directionForward = "forward"
	directionBack    = "back"
)
