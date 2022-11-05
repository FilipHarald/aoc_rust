use std::env;
use std::process;

use aoc_rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // TODO: either loop OR do single puzzle (part)

    let file_name = config.year.to_owned() + "_" + &config.day + "_" + &config.part;
    let input = aoc_rust::read_input_file(&config.assets_folder_path, &file_name).unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    let result = aoc_rust::solve_one_part(&config.year, &config.day, &config.part, &input);
    println!("{}", result);
}
