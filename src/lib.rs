use std::path::Path;
use std::fs;
use std::error::Error;
pub struct Config {
    pub in_file: String,
    pub out_file: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
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

    pub fn check_if_json(&self) -> bool {
        let in_path = Path::new(&self.in_file);
        let out_path = Path::new(&self.out_file);

        let in_ext = match in_path.extension() {
            Some(ext) => ext,
            None => return false,
        };

        let out_ext = match out_path.extension() {
            Some(ext) => ext,
            None => return false,
        };

        return in_ext == out_ext
    }
}

pub fn read(path: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    Ok(contents)
}

pub fn write(path: String, data: String) -> Result<(), Box<dyn Error>> {
    fs::write(path,data)?;

    Ok(())
}

pub fn rm_specials(data: String) -> String {
    let tokens = ['\t','\n'];
    let mut ret_string = String::new();
    for chr in data.chars() {
        if !(tokens.contains(&chr)) {
            ret_string.push(chr);
        };
    };

    ret_string
}

pub fn j_fmt(content: String) -> String {
    //first need to remove all whitespace and newlines
    let removed = rm_specials(content);
    let mut ret_string = String::new();
    let mut tab_count: u32 = 0;
    let mut in_string: bool = false;

    for chr in removed.chars() {
        if ret_string.ends_with('\n') {
            if tab_count > 0 {
                for _ in [1..tab_count] {
                    ret_string.push('\t');
                }
            }
        }
        if !in_string {
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
                '"' => {
                    in_string = true;
                    ret_string.push(chr);
                }
                _ => {
                    ret_string.push(chr);
                }
            }
        } else {
            if chr != '"' {
                ret_string.push(chr);
            } else {
                in_string = false;
                ret_string.push(chr);
            }
        }
        
    }

    ret_string
}