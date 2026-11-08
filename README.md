<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/), one folder per year.

Each year is a copy of [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust), trimmed down to just the code.

## Repository layout

```
.devcontainer/        shared dev container for every year
Cargo.toml            Cargo workspace + shared crate versions
Cargo.lock            one lockfile for every year
scripts/new-year.sh   adds a new year from the upstream template
2023/ 2024/ 2025/     one template project per year
  .cargo/config.toml  cargo aliases (solve, all, ...) and AOC_YEAR
  src/bin/NN.rs       solution for day NN
  data/examples/      example inputs (committed)
  data/inputs/        real puzzle inputs (git-ignored)
  data/puzzles/       puzzle descriptions (git-ignored)
```

Every command below is run from inside a year folder, e.g. `cd 2025`. The aliases, `AOC_YEAR` and the `data/` paths are all resolved from the current directory, and each year builds into its own `YYYY/target/`. Don't run `cargo build` from the repo root: the day binaries of different years have the same names and overwrite each other in the root `target/`.

## Setup

### 1. Session cookie (needed to download inputs)

Puzzle inputs are personal and [must not be committed](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3), so they are downloaded with [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) using your Advent of Code session cookie.

1. Log in at [adventofcode.com](https://adventofcode.com/), open the browser developer tools (F12), and copy the value of the `session` cookie (_Application_ / _Storage_ tab → _Cookies_).
2. Export it on the host, e.g. in `~/.zshrc`:

   ```sh
   export ADVENT_OF_CODE_SESSION="<cookie value>"
   ```

3. Restart VS Code from a shell that has the variable set. The dev container passes `ADVENT_OF_CODE_SESSION` through, and `aoc-cli` reads it directly.

Outside the container you can instead save the cookie to `~/.adventofcode.session` and install the CLI with `cargo install aoc-cli`.

The cookie expires after roughly a month. When downloads start failing with `Invalid session cookie`, copy a fresh one.

### 2. Dev container

Open the repo root in VS Code and run **Dev Containers: Rebuild and Reopen in Container**. The container (`.devcontainer/`) includes:

- Rust (stable), `aoc-cli`, `bacon`, `hyperfine`, `cargo flamegraph`, `lldb`
- z3 and CBC system libraries (for the `z3` crate and `good_lp`'s `coin_cbc` solver)
- VS Code extensions: rust-analyzer, CodeLLDB, EditorConfig, Claude Code, Codex

## Daily workflow

```sh
cd 2025

cargo download 1     # fetch input + puzzle description for day 1
cargo scaffold 1     # create src/bin/01.rs and empty data files
# paste the example from the puzzle into data/examples/01.txt
bacon test -- --bin 01   # re-runs the day's tests on every save
cargo solve 1        # run against the real input
```

Example files end with a newline, just like downloaded inputs (`.editorconfig` enforces it). A solution that trips over the trailing newline fails its test instead of only failing on the real input. Split input with `.lines()` (or `.trim()` first) rather than `.split("\n")`.

### ➡️ Download input & description for a day

```sh
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

The year comes from `AOC_YEAR` in the year's `.cargo/config.toml`. To fetch every input for a past year in one go:

```sh
for d in $(seq 1 25); do cargo download $d; done   # use 12 from 2025 on
```

### ➡️ Read a puzzle description in the terminal

```sh
cargo read <day>
```

### ➡️ Scaffold a day

```sh
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

`cargo scaffold <day> --download` scaffolds and downloads in one step. During December, `cargo today` does both for the current day.

If a day has several example inputs, name them `01-2.txt`, `01-3.txt`, ... and load them in tests with `advent_of_code::template::read_file_part("examples", DAY, 2)`.

### ➡️ Solve a day

```sh
cargo solve <day>

# output:
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

- `--release`: run an optimized build of the day (the default is a debug build, which is slower but catches integer overflows).
- `--dhat`: profiles heap allocations and writes `dhat-heap.json` (view it with [dh_view](https://nnethercote.github.io/dh_view/dh_view.html)).
- `--submit <part>`: submits the answer for part 1 or 2 through `aoc-cli`.

### ➡️ Run all days

```sh
cargo all            # add --release for optimized builds
```

Days without an input file show as `Not solved`.

### ➡️ Benchmark

```sh
cargo time           # time days that have no stored timing yet
cargo time --all     # re-time everything
cargo time <day>     # time one day
```

Each solution runs between 10 and 10,000 times in release mode, and the average is printed. Add `--store` to save timings to `data/timings.json` (git-ignored). With `--store`, the template also tries to write a table to the year's `README.md`. The year folders have no README, so that step prints `Failed to store updated benchmarks.`, which is harmless.

### ➡️ Test, format, lint

```sh
cargo test                     # all days
cargo test --bin 01 part_one   # one part of one day
cargo fmt
cargo clippy --all-targets
bacon clippy                   # clippy on every save
```

Lints are shared through `[workspace.lints]` in the root `Cargo.toml`. On top of clippy's defaults, solutions get warnings for `print!`/`println!`/`eprintln!` and `dbg!`, so debug output doesn't get left in. The template's runner, which prints results, is exempted in `src/lib.rs` and `src/main.rs`. rust-analyzer runs clippy too, so the warnings show up in the editor.

## Shared crates

All years depend on the same crates, with versions set once in the root `Cargo.toml` under `[workspace.dependencies]`:

| Crate | Why |
| --- | --- |
| [itertools](https://crates.io/crates/itertools) | Iterators do a lot of the heavy lifting |
| [bumpalo](https://crates.io/crates/bumpalo) | Arena allocation; fast when nothing needs a complex destructor |
| [rayon](https://crates.io/crates/rayon) | Most puzzles are trivially parallel |
| [regex](https://crates.io/crates/regex) | Parsing puzzle input |
| [rustc-hash](https://crates.io/crates/rustc-hash) | Faster `HashMap`/`HashSet` hasher |
| [pathfinding](https://crates.io/crates/pathfinding) | BFS, Dijkstra, A*, flows, ... |
| [num](https://crates.io/crates/num) | gcd/lcm, big integers, rationals |
| [good_lp](https://crates.io/crates/good_lp) | Linear / integer programming (`microlp` and `coin_cbc` solvers) |
| [z3](https://crates.io/crates/z3) | SMT solver for constraint puzzles |

To add a crate for every year, add it to `[workspace.dependencies]` and add `name.workspace = true` to each year's `Cargo.toml`.

## Adding a new year

```sh
./scripts/new-year.sh 2026
cd 2026
cargo download 1 && cargo scaffold 1
```

The script clones the latest upstream template into `2026/` and strips everything except the code. It then hooks the folder into the workspace: package name `aoc_2026`, the shared crates, its own `target/`, and `AOC_YEAR = "2026"`. The workspace picks up any `20*` folder automatically.

## Updating the template in existing years

The template code lives in each year's `src/template/`, `src/main.rs`, `src/lib.rs`, `src/template.txt` and `src/template_last_day.txt`. Solutions only live in `src/bin/`. To update to a newer upstream version, copy those files from a fresh clone of the template into each year. Then re-add the `#[allow(clippy::print_stdout, clippy::print_stderr)]` lines at the top of `src/main.rs` and above `pub mod template;` in `src/lib.rs`.

## Common pitfalls

- **Integer overflows:** integer overflow panics in debug builds but silently wraps with `--release`. Use `u64`/`i64` (the default in `src/template.txt`) when answers can get large.
- **Expired session:** downloads fail with `Invalid session cookie` → refresh `ADVENT_OF_CODE_SESSION` (see [Setup](#1-session-cookie-needed-to-download-inputs)).
