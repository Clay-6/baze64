use std::{
    fs::File,
    io::{Read, Write},
};

use baze64::Base64String;
use clap::Parser;
use cli::{Args, Command};
use color_eyre::{eyre::eyre, Result};

mod cli;

fn main() {
    color_eyre::install().unwrap();

    if let Err(e) = baze64() {
        eprintln!("Error: {e}")
    }
}

fn baze64() -> Result<()> {
    match Args::parse().cmd {
        Command::Encode {
            string,
            file,
            alphabet,
            no_padding,
        } => {
            let data = if let Some(txt) = string {
                txt.as_bytes().to_vec()
            } else if let Some(path) = file {
                let mut f = File::open(path)?;
                let mut buf = vec![];
                f.read_to_end(&mut buf)?;

                buf
            } else {
                return Err(eyre!(
                    "Either provide a string or use `-f <FILE>` to provide a file to encode"
                ));
            };

            let b64 = Base64String::encode_with(&data, alphabet)?;
            println!(
                "{}",
                if !no_padding {
                    b64.to_string()
                } else {
                    b64.without_padding()
                }
            );
        }
        Command::Decode {
            base64,
            output,
            alphabet,
            hex,
            bytes,
        } => {
            let decoded = Base64String::from_encoded(&base64, alphabet).decode()?;

            if let Some(path) = output {
                let mut f = File::create(path)?;
                f.write_all(&decoded)?;
            } else if hex {
                print!("0x{:0>2X}", decoded.first().unwrap_or(&0));
                decoded.iter().skip(1).for_each(|b| print!("{b:0>2X}"));
                println!();
            } else if bytes {
                decoded.iter().for_each(|b| print!("{b:0>8b}"));
                println!();
            } else {
                println!("{}", String::from_utf8_lossy(&decoded))
            }
        }
    }

    Ok(())
}
