def main():
  n = input()
  print(solve(n))

def solve(n):
  d = [0, 1]
  xs = [int(x) for x in reversed(n)]
  for x in xs:
    p, q = d
    d[0] = min(p+x, q+x+1)
    d[1] = min(p+10-x, q+9-x)
  return min(d[0], d[1]+1)

if __name__ == '__main__':
  main()