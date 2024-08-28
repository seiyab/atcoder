import subprocess, os, time
from pathlib import Path

def main():
    my_env = os.environ.copy()
    my_env["SCORE"] = "1"
    subprocess.run(["rustc", "./main.rs"], check=True)
    score = 0
    worst_time = 0
    fs = [p for p in  Path("./in").iterdir() if not p.is_dir()]
    fs = sorted(fs, key=lambda x: x.name)[:30]
    for i, p in enumerate(fs):
        with open(p, 'r') as f:
            d = f.read()
        start = time.time()
        o = subprocess.run("./main", input=d, check=True, capture_output=True, text=True, env=my_env)
        end = time.time()
        elapsed = int((end - start) * 1000)
        out = o.stdout
        local_score = int(out)
        if i < 7:
            print(f"{p.name}: {local_score}, {elapsed}")
        score += local_score
        worst_time = max(worst_time, elapsed)
    print("--------------------")
    print(f"Average: {score / len(fs)}")
    print(f"Worst time: {worst_time} ms")

main()