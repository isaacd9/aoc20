package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func read() []int {
	s := bufio.NewScanner(os.Stdin)

	var arr []int
	for s.Scan() {
		line := s.Text() // Println will add back the final '\n'
		num, err := strconv.Atoi(line)
		if err != nil {
			log.Fatalf("reading standard input: %+v", err)
		}

		arr = append(arr, num)
	}

	if err := s.Err(); err != nil {
		log.Fatalf("reading standard input: %+v", err)
	}

	return arr
}

const target = 2020

func findTwoNumbers(arr []int, t int) int {
	m := map[int]int{}
	for _, a := range arr {
		m[a] = t - a
	}

	var twoResult int
	for _, v := range m {
		_, ok := m[v]
		if ok {
			log.Printf("%d+%d=%d", m[v], v, m[v]+v)
			twoResult = m[v] * v
		}
	}

	return twoResult
}

func main() {
	arr := read()

	for _, a := range arr {
		tt := target - a
		product := findTwoNumbers(arr, tt)
		if product > 0 {
			log.Printf("%d", product*a)
		}
	}
}
