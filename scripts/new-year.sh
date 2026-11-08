#!/usr/bin/env bash
# Clone the advent-of-code-rust template into a new year folder and wire it into the root workspace.
# Usage: ./scripts/new-year.sh <year> [template-git-url]
set -euo pipefail

year="${1:?usage: $0 <year> [template-git-url]}"
template="${2:-https://github.com/fspoettel/advent-of-code-rust.git}"
root="$(cd "$(dirname "$0")/.." && pwd)"
dir="$root/$year"

if [[ -e "$dir" ]]; then
    echo "error: $dir already exists" >&2
    exit 1
fi

git clone --depth 1 "$template" "$dir"

# These live at the repo root instead
rm -rf "$dir/.git" "$dir/.devcontainer" "$dir/.vscode" "$dir/Cargo.lock" "$dir/.editorconfig" \
    "$dir/.github" "$dir/.assets" "$dir/README.md" "$dir/.gitignore" "$dir/LICENSE"

python3 - "$root" "$year" <<'PY'
import re, sys, tomllib
root, year = sys.argv[1], sys.argv[2]
d = f"{root}/{year}"

# Every year depends on all shared workspace crates; the template's chrono/dhat stay optional
with open(f"{root}/Cargo.toml", "rb") as f:
    shared = tomllib.load(f)["workspace"]["dependencies"]
template_deps = ["chrono", "dhat", "pico-args", "tinyjson"]
optional = {"chrono", "dhat"}

def dep(name):
    return f"{name} = {{ workspace = true, optional = true }}" if name in optional else f"{name}.workspace = true"

deps = "[dependencies]\n\n# Template dependencies\n"
deps += "".join(dep(n) + "\n" for n in template_deps if n in shared)
deps += "\n# Solution dependencies\n"
deps += "".join(dep(n) + "\n" for n in shared if n not in template_deps)

p = f"{d}/Cargo.toml"
with open(p) as f:
    s = f.read()
s = s.replace('name = "advent_of_code"', f'name = "aoc_{year}"', 1)
s = s.replace("[lib]\ndoctest = false\n", (
    '[lib]\nname = "advent_of_code"\ndoctest = false\n\n'
    "# Keep the runner binary's name independent of the package name (used by default-run)\n"
    '[[bin]]\nname = "advent_of_code"\npath = "src/main.rs"\n'
), 1)
s = re.sub(r"\[profile\.dhat\]\n(?:[^\[\n].*\n)*\n?", "", s)
s = s[: s.index("[dependencies]")] + "[lints]\nworkspace = true\n\n" + deps
with open(p, "w") as f:
    f.write(s)

# The template's runner prints results on purpose; the workspace print lints are for solutions
p = f"{d}/src/lib.rs"
with open(p) as f:
    s = f.read()
s = s.replace("pub mod template;", (
    "// The template's runner prints results on purpose\n"
    "#[allow(clippy::print_stdout, clippy::print_stderr)]\npub mod template;"
), 1)
with open(p, "w") as f:
    f.write(s)

p = f"{d}/src/main.rs"
with open(p) as f:
    s = f.read()
with open(p, "w") as f:
    f.write("#![allow(clippy::print_stdout, clippy::print_stderr)]\n\n" + s)

p = f"{d}/.cargo/config.toml"
with open(p) as f:
    s = f.read()
s = re.sub(r'AOC_YEAR = "\d+"', f'AOC_YEAR = "{year}"', s)
s = s.rstrip("\n") + (
    "\n\n# Per-year target dir so day binaries from different years do not collide in the workspace target\n"
    '[build]\ntarget-dir = "target"\n'
)
with open(p, "w") as f:
    f.write(s)
PY

echo "Created $year/. Next: cd $year && cargo scaffold 1"
