#!/usr/bin/env python3
"""
Filter compile_commands.json based on target patterns.

Usage:
    filter_compile_commands.py <input.json> <output.json> <pattern1> [pattern2] ...

Patterns are matched against the 'file' field in compile_commands.json.
A file is included if it matches ANY of the include patterns (substring match).
Patterns starting with '!' are exclusion patterns - files matching them are excluded.
Patterns starting with '~!' exclude entries where the command field contains the pattern.
Patterns starting with '~' (without !) remove matching text from the command field.
"""

import json
import sys


def main():
    if len(sys.argv) < 4:
        print(f"Usage: {sys.argv[0]} <input.json> <output.json> <pattern1> [pattern2] ...", file=sys.stderr)
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]
    all_patterns = sys.argv[3:]

    include_patterns = [p for p in all_patterns if not p.startswith('!') and not p.startswith('~')]
    exclude_patterns = [p[1:] for p in all_patterns if p.startswith('!') and not p.startswith('~')]
    command_excludes = [p[2:] for p in all_patterns if p.startswith('~!')]
    command_filters = [p[1:] for p in all_patterns if p.startswith('~') and not p.startswith('~!')]

    with open(input_file, 'r') as f:
        commands = json.load(f)

    filtered = []
    for cmd in commands:
        file_path = cmd.get('file', '')
        command = cmd.get('command', '')
        if any(pattern in file_path for pattern in exclude_patterns):
            continue
        if any(pattern in command for pattern in command_excludes):
            continue
        if any(pattern in file_path for pattern in include_patterns):
            if command_filters:
                cmd = cmd.copy()
                for cf in command_filters:
                    command = command.replace(cf, '')
                cmd['command'] = command
            filtered.append(cmd)

    with open(output_file, 'w') as f:
        json.dump(filtered, f, indent=2)

    print(f"Filtered {len(commands)} -> {len(filtered)} entries")


if __name__ == '__main__':
    main()
