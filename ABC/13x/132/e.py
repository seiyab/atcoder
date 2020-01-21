import heapq

def main():
    n, m = map(int, input().split())
    uvs = [tuple(map(int, input().split())) for _ in range(m)]
    s, t = map(int, input().split())
    edges = [{} for _ in range(n*3)]
    for u, v in uvs:
        x, y = u-1, v-1
        for i in range(3):
            c = 1 if i == 0 else 0
            z = y + n*((i+1)%3)
            edges[x + n*i][z] = c
    l = dijkstra(edges, s-1, t-1)
    if l is None:
        print(-1)
    else:
        print(l)

def dijkstra(graph, start, end):
    q = []
    heapq.heapify(q)
    visited = set()

    for next_, cost in graph[start].items():
        heapq.heappush(q, (cost, next_))

    while len(q) > 0:
        node_cost, current = heapq.heappop(q)
        if current in visited:
            continue
        if current == end:
            return node_cost

        visited.add(current)
        for next_, path_cost in graph[current].items():
            heapq.heappush(q, (node_cost + path_cost, next_))

    return None

main()