package main

import ("fmt"; "io/ioutil"; "log"; "strings"; "strconv")

func handle(err error) {
	if err != nil {
		log.Fatal(err)
	}
}
func Int(str string) int {
	ret, err := strconv.Atoi(str)	
	handle(err)
	return ret
}
func part1(filename string) int {
	content, err := ioutil.ReadFile(filename)
	handle(err)
	lines := strings.Split(string(content), "\n")
	increasing := 0
	prev := Int(lines[0])
	for _, line := range lines[1:] {
		curr := Int(line) 
		if curr > prev {
			increasing += 1
		}
		prev = curr
	}
	return increasing
}

func part2(filename string) int {
	content, err := ioutil.ReadFile(filename)
	handle(err)
	lines := strings.Split(string(content), "\n")
	increasing := -1 
	prev := 0
	for i, _ := range lines[2:] {
		a := Int(lines[i])
		b := Int(lines[i+1])
		c := Int(lines[i+2])
		curr := a + b + c 
		if curr > prev {
			increasing += 1
		}
		prev = curr
	}
	return increasing
}


func main() {
	fmt.Printf("Part 1: %d\n", part1("inputday1.txt"))
	fmt.Printf("Part 2: %d\n", part2("inputday1.txt"))
}