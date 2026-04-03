use clap::Parser;
use std::path::PathBuf;
use std::process;
mod solver;

/// A Simple CLI SAT Based Sudoku Solver
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the input file
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Path of the output file
    #[arg(short, long, value_name = "FILE", default_value = "solved.txt")]
    output: String,
}

fn main() {
    let args = Args::parse();

    if !args.input.exists() {
        eprintln!("Error: Input file '{:?} not found.", args.input);
        process::exit(1);
    }
    let input_str = args.input.to_str().unwrap_or_else(|| {
        eprintln!("Error: Invalid input path encoding.");
        process::exit(1)
    });

    let steno = solver::Steno::new(input_str, &args.output);

    println!("Processing puzzles from: {}", input_str);
    println!("Results will be saved to: {}", args.output);

    match steno.writer() {
        Ok(_) => println!("Successfully processed all puzzles."),
        Err(e) => {
            eprintln!("IO Error: {}", e);
            process::exit(1);
        }
    }
}