use std::fs;
use std::env;
use std::process;
use std::path::Path;
use std::error::Error;

struct Config {
    in_file: String,
    out_file: String,
}

impl Config {
    fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();

        let in_file = match args.next() {
            Some(arg) => arg,
            None => return Err("No input file supplied!"),
        };

        let out_file = match args.next() {
            Some(arg) => arg,
            None => return Err("No output file supplied!"),
        };

        Ok(Config {
            in_file,
            out_file,
        })
    }
}

fn check_if_json(cfg: &Config) -> bool {
    let in_path = Path::new(&cfg.in_file);
    let out_path = Path::new(&cfg.out_file);

    let in_ext = match in_path.extension() {
        Some(ext) => ext,
        None => return false,
    };

    let out_ext = match out_path.extension() {
        Some(ext) => ext,
        None => return false,
    };

    if in_ext == out_ext {
        return true;
    } else {
        return false;
    }
}

fn read(in_file: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(in_file)?;

    Ok(contents)
}

fn write_file(out_file: String, data: String) -> Result<(), Box<dyn Error>> {
    fs::write(out_file,data)?;

    Ok(())
}

fn rm_specials(content: String) -> String {
    let tokens = ['\t','\n'];
    let mut ret_string = String::new();
    for chr in content.chars() {
        if !(tokens.contains(&chr)) {
            ret_string.push(chr);
        };
    };

    ret_string
}

fn j_fmt(content: String) -> String {
    //first need to remove all whitespace and newlines
    let removed = rm_specials(content);
    let mut ret_string = String::new();
    let mut tab_count: u32 = 0;
    for chr in removed.chars() {
        if ret_string.ends_with('\n') {
            if tab_count > 0 {
                for _ in [1..tab_count] {
                    ret_string.push('\t');
                }
            }
        }

        match chr {
            '{' => {
                ret_string.push(chr);
                ret_string.push('\n');
                tab_count += 1;
            }
            '}' => {
                ret_string.push('\n');
                ret_string.push(chr);
                tab_count -= 1;
            }
            ':' => {
                // ret_string.push(' ');
                ret_string.push(chr);
                ret_string.push(' ');
            }
            '[' => {
                ret_string.push(chr);
                ret_string.push('\n');
                tab_count += 1;
            }
            ']' => {
                ret_string.push('\n');
                ret_string.push(chr);
                tab_count -= 1;
            }
            ',' => {
                ret_string.push(chr);
                ret_string.push('\n');
            }
            _ => {
                ret_string.push(chr);
            }
        }
    }

    ret_string
}


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments! {err}");
        process::exit(1);
    });

    println!("{}", config.in_file);
    println!("{}", config.out_file);
    
    if check_if_json(&config) {
        //now we can read in from in file.
        let contents = read(config.in_file).unwrap_or_else( |err| { 
            eprintln!("File read error: {}",err);
            process::exit(1);
        });

        let contents = rm_specials(contents);
        let contents = j_fmt(contents);

        if let Err(e) = write_file(config.out_file,contents) {
            eprintln!("Problem writing file! {e}");
            process::exit(1);
        }

        } else {
        eprintln!("Please select json files!");
        process::exit(1);
    }

}
