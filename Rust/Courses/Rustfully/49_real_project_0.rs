use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Config {
    pattern: String,
    files: Vec<String>,
    case_insensitive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Usage: {} pattern file (or files [-i])", args[0]));
        }

        let mut case_insensitive = false;
        let mut non_flag_args = Vec::new();

        for arg in &args[1..] {
            if arg == "-i" {
                case_insensitive = true;
            } else {
                non_flag_args.push(arg.clone());
            }
        }

        if non_flag_args.len() < 2 {
            return Err("Error: Pattern and at least one file required".into());
        }

        Ok(Config {
            pattern: non_flag_args[0].clone(),
            files: non_flag_args[1..].to_vec(),
            case_insensitive,
        })
    }
}

fn grep(reader: &mut BufReader<File>, pattern: &str, case_insensitive: bool) -> io::Result<()> {
    let mut line = String::new();

    while reader.read_line(&mut line)? != 0 {
        let matched = if case_insensitive {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };

        if matched {
            print!("{}", line);
        }

        line.clear();
    }

    Ok(())
}

fn grep_file(file: &str, config: &Config) {
    match File::open(file) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            if let Err(e) = grep(&mut reader, &config.pattern, config.case_insensitive) {
                eprintln!("Error reading '{}': {}", file, e);
            }
        }
        Err(e) => {
            eprintln!("Could not open file '{}': {}", file, e);
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;

    for file in &config.files {
        grep_file(file, &config);
    }

    Ok(())
}
