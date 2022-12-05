use std::error::Error;
use std::fs;
mod puzzles;

pub struct Config {
    pub assets_folder_path: String,
    pub year: String,
    pub day: String,
    pub part: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            return Err("not enough arguments");
        }

        let assets_folder_path = args[1].clone();
        let year = args[2].clone();
        let day = args[3].clone();
        let part = args[4].clone();

        Ok(Config {
            assets_folder_path,
            year,
            day,
            part
        })
    }
}

pub fn read_input_file(assets_folder_path: &str, puzzle_id: &str) -> Result<String, Box<dyn Error>> {
    let file_path = assets_folder_path.to_owned() + "/" + puzzle_id + ".input.txt";
    let contents = fs::read_to_string(file_path)?.trim_end().to_owned();
    // TODO: incorrect file path error msg should include path
    return Ok(contents);
}

pub fn solve_one_part(year: &str, day: &str, part: &str, input: &str) -> String {
    // TODO: metrics start
    let result = puzzles::solve(&year, &day, &part, &input);
    // TODO: metrics stop
    // TODO: return metrics also?
    return result;
}


