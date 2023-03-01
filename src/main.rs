use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::ffi::OsString;
use std::error::Error;

pub struct ToUnix;

fn main() {
    let file_path = get_first_arg().expect("You need to supply the file path as an argument.");
    println!("Converting file to unix line endings...");
    ToUnix::convert(file_path.to_str().unwrap());
    println!("Converted!");
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

impl ToUnix {
    fn is_ascii(contents: &str) -> bool {
        let mut ascii = true;
        for c in contents.chars() {
            let code = c as i32;
            if code > 127 {
                ascii = false;
                break;
            }
        }
        ascii
    }

    fn is_dos_eol(contents: &str) -> bool {
        let mut dos_eol = false;
        for c in contents.chars() {
            if c == '\r' {
                dos_eol = true;
                break;
            }
        }
        dos_eol
    }

    fn to_unix_line_endings(contents: &str) -> Vec<String> {
        let mut ucontents = Vec::new();
        for c in contents.chars() {
            if c != '\r' {
                ucontents.push(format!("{}", c));
            }
        }
        ucontents
    }

    pub fn convert(filename: &str) {
        let mut input = File::open(filename).unwrap();
        let mut contents = String::new();
        let _ = input.read_to_string(&mut contents);
        let ascii = ToUnix::is_ascii(&contents);
        let dos_eol = ToUnix::is_dos_eol(&contents);

        if ascii && dos_eol {
            let converted = ToUnix::to_unix_line_endings(&contents);
            let mut w = File::create(filename).unwrap();
            let _ = w.write_all(converted.join("").as_bytes());
        }
    }

}
