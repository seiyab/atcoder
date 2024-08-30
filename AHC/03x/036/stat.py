import subprocess, os
from pathlib import Path
from collections import Counter

def main():
    my_env = os.environ.copy()
    subprocess.run(["cargo", "build", "--release", "--offline"], check=True)
    fs = [p for p in Path("./in").iterdir() if not p.is_dir()]
    fs = sorted(fs, key=lambda x: x.name)[:5]
    for i, p in enumerate(fs):
        with open(p, 'r') as f:
            d = f.read()
        o = subprocess.run("./target/release/ahc036", input=d, check=True, capture_output=True, text=True, env=my_env)
        out = o.stdout
        _n, _m, _t, la, lb = d.split("\n")[0].split(" ")
        As, *steps = map(lambda x: x.split(" "), out.split("\n"))
        As = list(map(int, As))
        steps = list(map(lambda x: [x[0], *map(int, x[1:])], steps))
        path = [s[1] for s in steps if s[0] == "m"]
        sigs = [s[1:] for s in steps if s[0] == "s"]

        print(f"-------------------- {p.name}")
        print(f"la: {la}, lb: {lb}")
        print(f"----- As")
        print("most common node: ", Counter(As).most_common(10))
        print("most common edge: ", Counter(map(lambda x: tuple(sorted(x)), zip(As, As[1:]))).most_common(10))
        print(f"----- steps")
        print(f"unique nodes: {len(set(s[1] for s in steps if s[0] == 'm'))}")
        print(f"path length: {len(path)}")
        print(f"signals: {len(sigs)}")
        print(f"most common node: ", Counter(path).most_common(10))
        print(f"least common node: ", Counter(path).most_common()[-10:])
        print()
        

main()