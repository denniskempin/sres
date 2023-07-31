#!/usr/bin/env python3
#
# This directory contains test roms and their corresponding execution trace log
# from BSNES.
#
# This script will process them into a usable format:
#
# - Compile all *.asm files
# - Trim trailing infinite loop and compress all .log files

from pathlib import Path
from typing import List
import subprocess

def trim_infinite_loop(path: Path):
    lines: List[str] = []
    for line in path.open("r").readlines():
        lines.append(line)
        mnemonic = line[7:10]
        pc = line[0:6]
        operand_effective_address = line[23:29]
        if mnemonic == "jmp" and pc == operand_effective_address:
            break
    path.write_text("".join(lines))

for f in Path(__file__).parent.glob('*.log'):
    print(f"Processing trace {f}")
    trim_infinite_loop(f)
    subprocess.run(['xz', '--compress', '--force', f])

for f in Path(__file__).parent.glob('*.asm'):
    print(f"Processing rom {f}")
    subprocess.run(['bass', f])

