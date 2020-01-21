package main

import (
	"errors"
	"fmt"
)

func main() {
	t1, t2 := scanIntPair()
	a1, a2 := scanIntPair()
	b1, b2 := scanIntPair()
	c1, c2 := a1-b1, a2-b2
	ans, err := solve(c1, c2, t1, t2)
	if err != nil {
		fmt.Println("infinity")
	} else {
		fmt.Println(ans)
	}
}

func solve(c1 int64, c2 int64, t1 int64, t2 int64) (int64, error) {
	if c1 < 0 {
		c1, c2 = -c1, -c2
	}
	forward := c1 * t1
	backward := c2 * t2
	cycle := forward + backward
	if cycle > 0 {
		return 0, nil
	} else if cycle == 0 {
		return 0, errors.New("infinity")
	} else {
		repeat := forward / -cycle
		touch := forward%-cycle == 0
		if repeat <= 0 {
			return 1, nil
		} else if touch {
			return 1 + 2*repeat - 1, nil
		} else {
			return 1 + 2*repeat, nil
		}
	}
}

func scanIntPair() (int64, int64) {
	var a, b int64
	fmt.Scanf("%d %d", &a, &b)
	return a, b
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
