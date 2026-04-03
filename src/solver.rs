use std::{char, fs::File, io::{BufRead, BufReader,}};
use indicatif::{ProgressBar, ProgressStyle};
use varisat::{ExtendFormula, Solver, Var};
use std::fmt;
use std::io::Write;

pub struct Sudoku {
    pub grid : Vec<Vec<usize>>
}

impl fmt::Display for Sudoku  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "\n")?;
        for (r, row) in self.grid.iter().enumerate() {
            if r > 0 && r  % 3 == 0 {
                writeln!(f, "------+------+------")?;
            }

            for (c , &val) in row.iter().enumerate() {
                if c > 0 && c % 3 == 0 {
                    write!(f, "| ")?;
                }

                if val == 0 {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{} ", val)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "------+------+------")?;
        write!(f, "\n")?;
        Ok(())
    }
}
impl Sudoku {
    fn typecaster(ch : &char) -> Option<usize> {
        match ch {
            '.' => Some(0),
            _ => {
                match ch.to_digit(10) {
                    Some(x) => Some(x as usize),
                    _ => None
                }
            }
        }
    }

    fn preprocess_internal(text : &str) -> Vec<Vec<usize>> {
        let mut v : Vec<_>= Vec::new();
        let mut temp : Vec<usize> = Vec::new();
        for (_, char) in text.chars().enumerate() {
            if let Some(digit) = Self::typecaster(&char) {
                temp.push(digit);
            } else {
                continue;
            }

            if temp.len() == 9 {
                v.push(temp.clone());
                temp.clear();
            }
        }

        v
    }


    pub fn new(text : &str) -> Self {
        let grid = Self::preprocess_internal(text);
       Self {
            grid
        }
    }
}

pub struct SudokuSolver {
    pub sudoku : Sudoku,
}

impl fmt::Display for SudokuSolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       
        write!(f, "{}", self.sudoku)?;
        if let  Some(x) = self.solve() {
            write!(f, "Solution of the puzzle : ")?;
            write!(f, "{}", x)?;
        } else {
            writeln!(f, "No solution found")?;
        }

        Ok(())
    }
}


impl SudokuSolver {
    pub fn new(text : &str) -> Self{
        Self {
            sudoku : Sudoku::new(text),
        }
    }

    /// The entry point: takes a 9x9 grid and returns the solved 9x9 grid
    pub fn solve(&self) -> Option<Sudoku> {
        let input_grid = self.sudoku.grid.clone();
        let mut solver = Solver::new();

        Self::add_constraints(&mut solver);

        for r in 0..9 {
            for c in 0..9 {
                let val = input_grid[r][c];
                if val > 0 && val <= 9 {
                    solver.add_clause(&[Self::get_var(r + 1, c + 1, val).positive()]);
                }
            }
        }

        if solver.solve().unwrap() {
            let model = solver.model().unwrap();
            let mut solution = vec![vec![0;9];9];

            for i in 1..=9 {
                for j in 1..=9 {
                    for v in 1..=9 {
                        if model.contains(&Self::get_var(i, j, v).positive()) {
                            solution[i - 1][j -1] = v;
                            break;
                        }
                    }
                }
            }
            Some(Sudoku { grid: solution })
        } else {
            None
        }
    }

    fn get_var(r: usize, c: usize, v: usize) -> Var {
        let index = 81 * (r - 1) + 9 * (c - 1) + (v - 1);
        Var::from_index(index)
    }

    fn add_constraints(solver: &mut Solver) {
        for r in 1..=9 {
            for c in 1..=9 {
                let mut cell_clause = Vec::with_capacity(9);
                for v in 1..=9 {
                    cell_clause.push(Self::get_var(r, c, v).positive());
                }
                solver.add_clause(&cell_clause);

                for v1 in 1..=9 {
                    for v2 in (v1 + 1)..=9 {
                        solver.add_clause(&[
                            Self::get_var(r, c, v1).negative(),
                            Self::get_var(r, c, v2).negative(),
                        ]);
                    }
                }
            }
        }

        for i in 1..=9 {
            for v in 1..=9 {
                let mut row_clause = Vec::with_capacity(9);
                let mut col_clause = Vec::with_capacity(9);
                for j in 1..=9 {
                    row_clause.push(Self::get_var(i, j, v).positive());
                    col_clause.push(Self::get_var(j, i, v).positive());
                }
                solver.add_clause(&row_clause);
                solver.add_clause(&col_clause);
            }
        }

        for box_r in 0..3 {
            for box_c in 0..3 {
                for v in 1..=9 {
                    let mut box_clause = Vec::with_capacity(9);
                    for r in 1..=3 {
                        for c in 1..=3 {
                            box_clause.push(Self::get_var(box_r * 3 + r, box_c * 3 + c, v).positive());
                        }
                    }
                    solver.add_clause(&box_clause);
                }
            }
        }
    }
}

pub struct Steno {
    pub input: String,
    pub output: String,
}

impl Steno {
    pub fn new(path: &str, output: &str) -> Self {
        Self {
            output: output.to_string(),
            input: path.to_string(),
        }
    }

    pub fn reader(&self) -> std::io::Result<Vec<String>> {
        let file = File::open(&self.input)?;
        let reader = BufReader::new(file);
        let mut ans = Vec::new();

        for line in reader.lines() {

            let line = line?;
            let trimmed = line.trim().to_string();

            if !trimmed.is_empty() {
                ans.push(trimmed);
            }
        }

        Ok(ans)
    }

    pub fn writer(&self) -> std::io::Result<()> {

        let puzzles = self.reader()?;
        println!("Found {} puzzles", puzzles.len());
        let mut file = File::create(&self.output)?;
     
        let pb = ProgressBar::new(puzzles.len() as u64);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                    .unwrap()
                    .progress_chars("#>-"));
        for (i, elem) in puzzles.iter().enumerate() {
            let elem = elem.trim();
            let sudoku = SudokuSolver::new(elem) ;
            
            println!("--- Solving Puzzle #{} ---", i + 1);
            write!(file, "Puzzle #{} :", i + 1)?;
            writeln!(file, "{}", sudoku)?;
            pb.inc(1);
        }
        pb.finish_with_message("Done!");
        file.flush()?;
        Ok(())
    }
}
