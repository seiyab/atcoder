def main():
  h, w, k = map(int, input().split())
  s = [input() for _ in range(h)]
  ans = 0
  for i in range(2**h):
    for j in range(2**w):
      if z(s, b(j, w), b(i, h)) == k:
        ans += 1
  print(ans)


def z(s, row, col):
  return sum(
    sum(0 if s[i][j] == '.' or c or r else 1 for j, r in enumerate(row))
    for i, c in enumerate(col)
  )

def b(n, digit):
  return [n // (2**i) % 2 for i in range(digit)]

main()