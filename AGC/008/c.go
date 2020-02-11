
package main

import (
	"fmt"
)

const (
	i int = iota
	o
	t
	j
	l
)

func main() {
	as := scanInts(7)
	im := as[i] % 2
	jm := as[j] % 2
	lm := as[l] % 2

	ans := as[o] + as[i]-im + as[j]-jm + as[l]-lm
	if im+jm+lm == 2 && as[i]!=0 && as[j]!=0 && as[l]!=0{
		ans += 1
	}
	if im+jm+lm == 3 {
		ans += 3
	}
	fmt.Println(ans)
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