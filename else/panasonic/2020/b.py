h, w = map(int, input().split())
if h == 1 or w == 1:
    print(1)
else:
    z = w * (h // 2)
    print(z + (h%2)*((w+1)//2))