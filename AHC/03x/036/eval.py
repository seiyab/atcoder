import subprocess, os, time
from pathlib import Path

def main():
    my_env = os.environ.copy()
    check_panic = my_env.get("PANIC", "") == "1"
    if not check_panic:
        my_env["SCORE"] = "1"
    subprocess.run(["cargo", "build", "--release", "--offline"], check=True)
    score = 0
    worst_time = 0
    dir = Path("./in") if not check_panic else Path("./in_1000")
    fs = [p for p in  dir.iterdir() if not p.is_dir()]
    fs = sorted(fs, key=lambda x: x.name)
    if not check_panic:
        fs = fs[:50]
    for i, p in enumerate(fs):
        with open(p, 'r') as f:
            d = f.read()
        start = time.time()
        o = subprocess.run("./target/release/ahc036", input=d, check=True, capture_output=True, text=True, env=my_env)
        end = time.time()
        elapsed = int((end - start) * 1000)
        out = o.stdout
        _n, _m, _t, la, lb = d.split("\n")[0].split(" ")
        if check_panic:
            outl = len(out.split("\n")[0].split(" "))
            if outl != int(la):
                raise Exception(f"Invalid input: {p.name}, ({outl}, {la})")
        else:
            local_score = int(out)
            if i < 15 and not check_panic:
                print(f"{p.name} ({int(la):4d}, {int(lb):2d}): {local_score}, {elapsed}")
            score += local_score
            worst_time = max(worst_time, elapsed)
    if not check_panic:
        print("--------------------")
        print(f"Average: {int(score / len(fs))}")
        print(f"Worst time: {worst_time} ms")

main()