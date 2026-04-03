# SAT Sudoku Solver

A high-performance CLI tool written in **Rust** that solves Sudoku puzzles by reducing them to a **Boolean Satisfiability (SAT)** problem using the `varisat` engine.

---

## Installation

```bash
git clone git@github.com:Shivanshu-Singh-2203/suduko-solver.git
cd suduko-solver
cargo build --release
```

---

## Usage

```bash
./suduko-solver -i <INPUT_FILE> [OPTIONS]
```

### Options
* `-i, --input <FILE>`: **Required.** Path to input `.txt` file.
* `-o, --output <FILE>`: Path to output file (default: `solved.txt`).
* `-h, --help`: Show help.

---

## Input Requirements

**CRITICAL:** The solver assumes each puzzle is a single **81-character line**. 

* **Placeholders:** Use `0` or `.` for empty cells.
* **Format:** One puzzle per line, ending with a newline (`\n`).
* **Validation:** To check your input line count on Linux, run `wc -l <file>`.

**Example `p.txt`:**
```text
003020600900305001001806400008102900700000008006708200002609500800203009005010300
.2..5....9....4.3.........43.8....1....2........7......5..2......1.........7.8..
```

---

## Logic Overview
The solver maps the board to **729 Boolean variables**, enforcing:
1.  **Cell:** Exactly one digit per cell.
2.  **Row/Col:** Digits 1–9 appear exactly once per line.
3.  **Box:** Digits 1–9 appear exactly once per $3 \times 3$ subgrid.

---
