import subprocess
from pathlib import Path

def main():
    subprocess.run(["rustc", "./main.rs"], check=True)
    score = 0
    for p in Path("./in").iterdir():
        if p.is_dir():
            continue
        with open(p, 'r') as f:
            d = f.read()
        o = subprocess.run("./main", input=d, check=True, capture_output=True, text=True)
        out = o.stdout
        local_score = g(d, out)
        print(f"{p.name}: {local_score}")
        score += g(d, out)
    print("--------------------")
    print(f"Total: {score}")

def g(d, out):
    lines = d.split("\n")
    o = out.split("\n")
    n, m, t = map(int, lines[0].split(" "))
    h = 2 * n * (n-1)
    lines = lines[1:]
    x, lines = get_mat(lines[:h]), lines[h:]
    x_init = x
    for i in range(t):
        a, o = get_mat(o[:n]), o[n:]
        u, lines = get_g(lines[:n]), lines[n:]
        v, lines = get_g(lines[:n-1]), lines[n-1:]
        x = harvest(x, a, u, v)
    
    xm = 0
    for i in range(m):
        m = 0
        for xx in x_init:
            m = max(m, xx[i])
        xm += m
    
    w = 0
    for xx in x:
        w = max(w, sum(xx))
    
    return int(10**6 * w / xm + 0.5)


def get_mat(lines):
    xs = []
    for l in lines:
        x = list(map(int, l.split(" ")))
        xs.append(x)
    return xs

def get_g(lines):
    xss = []
    for l in lines:
        g = l.split(" ")
        xs = []
        for z in g:
            x = list(map(int, z))
            xs.append(x)
        xss.append(xs)
    return xss

def harvest(x, a, us, vs):
    h = []
    for i, u in enumerate(us):
        for j, xx in enumerate(u):
            z = []
            for k, r in enumerate(xx):
                p = a[i][j+r]
                z.append(x[p][k])
            h.append(z)
    for i, v in enumerate(vs):
        for j, xx in enumerate(v):
            z = []
            for k, r in enumerate(xx):
                p = a[i+r][j]
                z.append(x[p][k])
            h.append(z)
    return h

main()