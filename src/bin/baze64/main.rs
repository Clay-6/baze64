use std::{
    fs::File,
    io::{Read, Write},
};

use baze64::{alphabet::Standard, Base64String};
use clap::Parser;
use cli::{Args, Command};

mod cli;

fn main() {
    match Args::parse().cmd {
        Command::Encode { string, file } => {
            let data = if let Some(txt) = string {
                txt.as_bytes().to_vec()
            } else if let Some(path) = file {
                let mut f = File::open(path).unwrap();
                let mut buf = vec![];
                f.read_to_end(&mut buf).unwrap();

                buf
            } else {
                panic!("Need either a string or a file");
            };
            let b64 = Base64String::<Standard>::encode(&data);

            println!("{}", b64);
        }
        Command::Decode { base64, output } => {
            let b64 = Base64String::<Standard>::from_encoded(&base64);
            let decoded = b64.decode();

            if let Some(path) = output {
                let mut f = File::create(path).unwrap();
                f.write_all(&decoded).unwrap();
            } else {
                println!("{}", String::from_utf8_lossy(&decoded))
            }
        }
    }
}
