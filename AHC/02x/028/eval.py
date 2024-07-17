import subprocess
from pathlib import Path

def main():
    subprocess.run(["rustc", "./main.rs"], check=True)
    score = 0
    for p in Path("./in").iterdir():
        if p.is_dir():
            continue
        # if p.name != "0000.txt":
        #     continue
        with open(p) as f:
            o = subprocess.run("./main", stdin=f, check=True, capture_output=True)
            out = o.stdout.decode('utf-8')
            score += g(out)
        
    print(score)

# NOTE: not perfect
def g(ans):
    v = 0
    ss = ans.split("\n")
    px, py = map(int, ss[0].split(" "))
    for s in ss[1:]:
        if s == "":
            continue
        x, y = map(int, s.split(" "))
        v += abs(px - x) + abs(py - y) + 1
        px, py = x, y
    return (10000 - v)

main()