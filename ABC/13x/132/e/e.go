package main

import (
	"fmt"
	"container/heap"
)

const infinity int64 = 1000 * 1000 * 1000 * 1000

func main() {
	var n, m int64
	fmt.Scanf("%d %d", &n, &m)
	uvs := make([]uv, m)
	for i := range iter(0, m) {
		fmt.Scanf("%d %d", &(uvs[i].u), &(uvs[i].v))
		uvs[i].u -= 1
		uvs[i].v -= 1
	}
	var s, t int64
	fmt.Scanf("%d %d", &s, &t)

	edgess := make([][]edge, n*3)
	for _, uv := range uvs {
		for i := range iter(0, 3) {
			from := uv.u + n*i
			edge := edge{
				from: from,
				to:   uv.v + n*((i+1)%3),
				cost: 1,
			}
			edgess[from] = append(edgess[from], edge)
		}
	}
	ans := dijkstra(n*3, edgess, s-1, t-1)
	if ans == infinity {
		fmt.Println(-1)
	} else {
		fmt.Println(ans / 3)
	}
}

type uv struct {
	u int64
	v int64
}

func dijkstra(n int64, edgess [][]edge, start int64, goal int64) int64 {
	pq := make(priorityQueue, 0)
	heap.Init(&pq)
	for _, edge := range edgess[start] {
		e := qElem{
			priority: edge.cost,
			content:  edge,
		}
		heap.Push(&pq, &e)
	}
	shortests := make([]int64, n)
	for i := range iter(0, n) {
		shortests[i] = infinity
	}
	shortests[start] = 0
	for pq.Len() > 0 {
		elem := heap.Pop(&pq).(*qElem)
		edge := elem.content.(edge)
		if shortests[edge.to] != infinity {
			continue
		}
		shortests[edge.to] = shortests[edge.from] + edge.cost
		if edge.to == goal {
			break
		}
		for _, nextEdge := range edgess[edge.to] {
			e := qElem{
				priority: shortests[edge.to] + nextEdge.cost,
				content:  nextEdge,
			}
			heap.Push(&pq, &e)
		}
	}
	return shortests[goal]
}

type edge struct {
	from int64
	to   int64
	cost int64
}

type qElem struct {
	priority int64
	content  interface{}
}

type priorityQueue []*qElem

func (pq priorityQueue) Len() int { return len(pq) }
func (pq priorityQueue) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}
func (pq priorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *priorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(*qElem))
}

func (pq *priorityQueue) Pop() interface{} {
	old := *pq
	item := old[len(old)-1]
	old[len(*pq)-1] = nil
	*pq = old[0 : len(old)-1]
	return item
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
