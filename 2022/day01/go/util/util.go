package util

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func readFile(caloriesChan chan string, path string) {
	defer close(caloriesChan)
	file, err := os.Open(path)
	if err != nil {
		log.Fatalf("Error opening file at %s", path)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		caloriesChan <- line
	}
}

func GetElves(elvesChan chan int) {
	defer close(elvesChan)
	caloriesChan := make(chan string)
	totalCalories := 0
	go readFile(caloriesChan, "./input.txt")
	for line := range caloriesChan {
		if len(line) == 0 {
			elvesChan <- totalCalories
			totalCalories = 0
		} else {
			calories, err := strconv.Atoi(line)
			if err != nil {
				log.Panicf("Could not convert %s to an integer", line)
			}
			totalCalories += calories
		}
	}
}
