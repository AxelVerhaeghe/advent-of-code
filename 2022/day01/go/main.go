package main

import (
	"day01/util"
	"log"
	"sort"
)

func main() {
	maxCalories := []int{0, 0, 0}
	elvesChan := make(chan int)
	go util.GetElves(elvesChan)
	for elf := range elvesChan {
		sort.Ints(maxCalories)
		maxCalories[0] = max(maxCalories[0], elf)
	}

	total := 0
	for _, calories := range maxCalories {
		total += calories
	}

	log.Println(total)
}
