n, k = map(int, input().split())
xs = [int(x) for x in input().split()]
fs = [int(f) for f in input().split()]

def lower_bound(l, r, ok):
    if ok(l):
        return l
    else:
        return unsafe_lower_bound(l, r, ok)

def unsafe_lower_bound(l, r, ok):
    if l + 1 >= r:
        return r
    m = (l + r) // 2
    if ok(m):
        return lower_bound(l, m, ok)
    else:
        return lower_bound(m, r, ok)

def ok(xs_asc, fs_desc, k, i):
    b = k
    for x, f in zip(xs_asc, fs_desc):
        over = x*f - i
        training = max(0, (over-1)//f+1)
        b -= training
    return b >= 0


def solve(xs, fs, k):
    xs_asc = sorted(xs)
    fs_desc = sorted(fs, reverse=True)
    l = 0
    r = xs_asc[-1] * fs_desc[0]
    return lower_bound(l, r, lambda i: ok(xs_asc, fs_desc, k, i))

print(solve(xs, fs, k))