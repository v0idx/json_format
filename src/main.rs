use std::env;
use std::process;

use json_format::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments! {err}");
        process::exit(1);
    });

    println!("Formatting {} > {}",config.in_file, config.out_file);
    
    if config.check_if_json() {
        //now we can read in from in file.
        let contents = json_format::read(config.in_file).unwrap_or_else( |err| { 
            eprintln!("File read error: {}",err);
            process::exit(1);
        });

        let contents = json_format::rm_specials(contents);
        let contents = json_format::j_fmt(contents);

        if let Err(e) = json_format::write(config.out_file,contents) {
            eprintln!("Problem writing file! {e}");
            process::exit(1);
        }

        } else {
        eprintln!("Please select json files!");
        process::exit(1);
    }

}
