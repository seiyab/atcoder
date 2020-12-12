from collections import defaultdict

def main():
  n, q = map(int, input().split())
  cs = [int(c) for c in input().split()]
  qs = [[int(x) for x in input().split()] for _ in range(q)]

  for o in solve(n, cs, qs):
    print(o)

def solve(n, cs, qs):
  uf = UnionFind(range(1, n+1))
  ds = [defaultdict(int) for _ in range(n+1)]
  for ii, c in enumerate(cs):
    i = ii + 1
    ds[i][c] += 1
  
  for (u, v, w) in qs:
    if u == 1:
      rv = uf.find(v)
      rw = uf.find(w)
      if rv == rw:
        continue

      uf.union(v, w)

      dg, dl = (ds[rv], ds[rw]) if len(ds[rv]) > len(ds[rw]) else (ds[rw], ds[rv])

      for k, z in dl.items():
        dg[k] += z

      rn = uf.find(v)
      ds[rn] = dg

    else:
      yield ds[uf.find(v)][w]
      

class UnionFind(object):
    def __init__(self, nodes):
        self.__parents = {node: None for node in nodes}

    def union(self, a, b):
        root_of_a = self.find(a)
        root_of_b = self.find(b)
        if root_of_a != root_of_b:
            self.__parents[root_of_a] = root_of_b

    def find(self, a):
        if self.__parents[a] is None:
            return a
        root = self.find(self.__parents[a])
        self.__parents[a] = root
        return root

    @property
    def parents(self):
        return {key: value for key, value in self.__parents.items()}



if __name__ == '__main__':
  main()