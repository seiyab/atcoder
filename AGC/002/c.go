package main

import (
	"errors"
	"fmt"
)

func main() {
	nl := scanInts(2)
	n, l := nl[0], nl[1]
	as := scanInts(n)

	knot, err := int64(0), errors.New("")
	for i := range iter(0, n-1) {
		if as[i]+as[i+1] >= l {
			knot, err = i, nil
			break
		}
	}
	if err != nil {
		fmt.Println("Impossible")
	} else {
		fmt.Println("Possible")
		for i := range iter(0, knot) {
			fmt.Println(i + 1)
		}
		for i := range iter(0, n-knot-2) {
			fmt.Println(n - 1 - i)
		}
		fmt.Println(knot + 1)
	}
}

func scanInts(length int64) []int64 {
	ints := make([]int64, length)
	for i := range iter(0, length) {
		var x int64
		_, _ = fmt.Scan(&x)
		ints[i] = x
	}
	return ints
}

func iter(l int64, r int64) chan int64 {
	var i int64 = l
	c := make(chan int64)
	run := func() {
		for i < r {
			c <- i
			i += 1
		}
		close(c)
	}
	go run()
	return c
}

func gcd(a int64, b int64) int64 {
	x, y := a, b
	for true {
		if x%y == 0 {
			break
		}
		x, y = y, x%y
	}
	return y
}

func slice_gcd(ints []int64) int64 {
	return reduce_int(gcd, ints)
}

func lcm(a int64, b int64) int64 {
	g := gcd(a, b)
	return a * b / g
}

func slice_lcm(ints []int64) int64 {
	return reduce_int(lcm, ints)
}

func reduce_int(f func(int64, int64) int64, ints []int64) int64 {
	ans := ints[0]
	for i := range iter(1, int64(len(ints))) {
		ans = f(ans, ints[i])
	}
	return ans
}
