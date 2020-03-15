a, b, c = map(int, input().split())
if False:
    print("No")
else:
    print("Yes" if 4*a*b < (c-a-b)**2 else "No")
