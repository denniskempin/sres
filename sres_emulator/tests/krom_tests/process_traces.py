#!/usr/bin/env python3
#
# Trims and compresses BSNES generated trace logs.

from pathlib import Path
import sys
import os
import subprocess

def trim_infinite_loop(path: Path):
    lines = []
    for line in path.open("r").readlines():
        lines.append(line)
        mnemonic = line[7:10]
        pc = line[0:6]
        operand_effective_address = line[23:29]
        if mnemonic == "jmp" and pc == operand_effective_address:
            break
    path.write_text("".join(lines))

for f in Path(__file__).parent.glob('*.log'):
    print(f"Processing {f}")
    trim_infinite_loop(f)
    subprocess.run(['xz', '--compress', '--force', f])
