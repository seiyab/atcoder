
def main():
    n = int(input())
    xs = list(map(int, input().split()))
    mod = 10 ** 9 + 7

    ps = [
        sum((x & (1<<i)) >> i for x in xs)
        for i in range(60)
    ]
    ms = [n - p for p in ps]

    ans = 0

    for i in range(60):
        s = 0
        for x in xs:
            b = (x & (1<<i)) >> i
            if b == 1:
                ps[i] -= 1
                s += ms[i]
            else:
                ms[i] -= 1
                s += ps[i]
        ans += s * (2**i % mod)
        ans = ans % mod

    print(ans)

main()