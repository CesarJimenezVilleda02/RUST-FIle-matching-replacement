use regex::Regex;
use std::fs;
use std::result::Result;
use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!(
        "{} - replace string with a new string",
        "Find and replace".green()
    );
    eprintln!(
        "{} <target string> <replacement string> <input_file> <output_file>",
        "Usage:".green()
    );
}

fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{}: Failed to read from file '{}': {}",
                "Error".red().bold(),
                args.input_file,
                e
            );
            std::process::exit(1);
        }
    };

    let replace_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{}: Failed to replace pattern '{}': {}",
                "Error".red().bold(),
                args.pattern,
                e
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output_file, replace_data) {
        Ok(_) => println!(
            "{}: Successfully wrote to file '{}'",
            "Success".green().bold(),
            args.output_file
        ),
        Err(e) => {
            eprintln!(
                "{}: Failed to write to file '{}': {}",
                "Error".red().bold(),
                args.output_file,
                e
            );
            std::process::exit(1);
        }
    }
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, rep).to_string())
}

fn parse_args() -> Arguments {
    // when we run programs, running the program is the first argument, we want everything else
    // args returns an iterator, we want to collect it into a vector
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() < 4 {
        print_help();
        eprintln!(
            "{}: Not enough arguments given, expected 4 got {}",
            "Error".red().bold(),
            args.len()
        );
        // error code 1 signifies the program did not run successfully
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

pub fn run() {
    let args = parse_args();

    read_and_write(&args);
}
