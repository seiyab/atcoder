package main

import (
	"fmt"
)

func main() {
	nm := scanInts(2)
	n, m := nm[0], nm[1]
	edges := make(map[int64][]int64)
	for _ = range iter(0, m) {
		ab := scanInts(2)
		edges[ab[0]] = append(edges[ab[0]], ab[1])
		edges[ab[1]] = append(edges[ab[1]], ab[0])
	}
	fmt.Println(solve(n, m, edges))
}

type color bool

const (
	blue  = true
	green = false
)

type elm struct {
	node  int64
	color color
}

func solve(n, m int64, edges map[int64][]int64) int64 {
	visited := make(map[int64]bool)
	ans := int64(0)
	for i := range iter(1, n+1) {
		if visited[i] {
			continue
		}
		blueGreen := make(map[color]map[int64]bool)
		blueGreen[blue] = make(map[int64]bool)
		blueGreen[green] = make(map[int64]bool)
		complete := false
		lv := make(map[int64]bool)
		edc := int64(0)
		stack := []elm{elm{node: i, color: blue}}
		blueGreen[blue][i] = true
		for len(stack) > 0 {
			l := len(stack)
			x := stack[l-1]
			stack = stack[:l-1]
			if lv[x.node] {
				continue
			}
			lv[x.node] = true
			visited[x.node] = true
			for _, nxt := range edges[x.node] {
				if blueGreen[x.color][nxt] {
					complete = true
				}
				nxtClr := !x.color
				blueGreen[nxtClr][nxt] = true
				stack = append(stack, elm{node: nxt, color: nxtClr})
			}
			edc += int64(len(edges[x.node]))
		}
		z := int64(len(lv))
		if complete {
			ans += z*(z-1)/2 - edc/2
		} else {
			ans += int64(len(blueGreen[blue])*len(blueGreen[green])) - edc/2
		}
	}
	return ans
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
	go func() {
		for i < r {
			c <- i
			i += 1
		}
		close(c)
	}()
	return c
}
