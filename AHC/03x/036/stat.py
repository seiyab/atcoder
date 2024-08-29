import subprocess, os
from pathlib import Path
from collections import Counter

def main():
    my_env = os.environ.copy()
    subprocess.run(["cargo", "build", "--release", "--offline"], check=True)
    fs = [p for p in  Path("./in").iterdir() if not p.is_dir()]
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

        print(f"-------------------- {p.name}")
        print(f"la: {la}, lb: {lb}")
        print(f"----- As")
        print("most common: ", Counter(As).most_common(5))
        print(f"----- steps")
        print(f"unique nodes: {len(set(s[1] for s in steps if s[0] == 'm'))}")
        print("m: ", sum(1 for s in steps if s[0] == "m"))
        print()
        

main()