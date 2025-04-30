from subprocess import check_output
from pathlib import Path
prev = ""

outputs = []

for width in range(40, 100):
    print(f"Width: {width}")
    output = check_output(["rustfmt", "input.rs", "--emit", "stdout", "-q", "--edition", "2024", "--style-edition", "2024", "--config", f"max_width={width},wrap_comments=true,comment_width={width}"])
    out_str = output.decode()
    if out_str != prev:
        prev = out_str
        outputs.append((width, out_str))

for (width, out_str) in outputs:
    Path(f"out/{width}_output.rs").write_text(out_str)

from json import dumps

Path("out/out.json").write_text(dumps(outputs))