from collections import defaultdict

def main():
  n, m = map(int, input().split())
  xys = [tuple(map(int, input().split())) for _ in range(n+m-1)]
  for p in solve(n, m, xys):
    print(p)

def solve(n, m, xys):
  es = defaultdict(set)
  fs = defaultdict(set)
  for x, y in xys:
    es[x].add(y)
    fs[y].add(x)

  cur = xys[0][1]
  while True:
    if len(fs[cur]) == 0:
      break
    cur = next(iter(fs[cur]))

  ps = [0 for _ in range(n+1)]
  q = Q()
  q.queue(cur)
  while True:
    cur = q.dequeue()
    if cur is None:
      break
    for w in es[cur]:
      if cur in fs[w]:
        fs[w].remove(cur)
      if len(fs[w]) == 0:
        ps[w] = cur
        q.queue(w)

  return ps[1:]

class Q:
  def __init__(self):
    self.q = []
    self.i = 0
  
  def queue(self, x):
    self.q.append(x)
  
  def dequeue(self):
    if self.i < len(self.q):
      i = self.i
      self.i += 1
      return self.q[i]
    else:
      return None


if __name__ == '__main__':
  main()