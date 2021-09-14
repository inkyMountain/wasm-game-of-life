package main

// This calls a JS function from Go.
func main() {
	// println("go process running", add(10000))
}

// This function is imported from JavaScript, as it doesn't define a body.
// You should define a function named 'main.add' in the WebAssembly 'env'
// module from JavaScript
// func add(x, y int) int`

//export multiply
func multiply(to int) int {
	sum := 0
	for i := 0; i < to; i++ {
		sum += i
	}
	return sum
}

//export add
func add(to int) int {
	sum := 0
	for i := 0; i < to; i++ {
		sum = i + sum
	}
	return sum
}

func xxx(to int) int {
	sum := 0
	for i := 0; i < to; i++ {
		sum += i
	}
	return sum
}
