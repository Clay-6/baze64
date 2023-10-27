use std::{
    fs::File,
    io::{Read, Write},
};

use baze64::Base64String;
use clap::Parser;
use cli::{Args, Command};
use color_eyre::{eyre::bail, Result};
use hex::FromHex;

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
            hex,
        } => {
            let data = if let Some(mut txt) = string {
                if hex {
                    if txt.len() % 2 != 0 {
                        txt = format!("0{txt}");
                    }
                    Vec::from_hex(txt)?
                } else {
                    txt.as_bytes().to_vec()
                }
            } else if let Some(path) = file {
                let mut f = File::open(path)?;
                let mut buf = vec![];
                f.read_to_end(&mut buf)?;

                buf
            } else {
                bail!("Either provide a string or use `-f <FILE>` to provide a file to encode");
            };

            let b64 = Base64String::encode_with(data, alphabet);
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
            let decoded = Base64String::from_encoded_with(base64, alphabet).decode()?;

            if let Some(path) = output {
                let mut f = File::create(path)?;
                f.write_all(&decoded)?;
                f.flush()?;
            } else if hex {
                print!("0x{:0>2X}", decoded.first().unwrap_or(&0));
                decoded.iter().skip(1).for_each(|b| print!("{b:0>2X}"));
            } else if bytes {
                decoded.iter().for_each(|b| print!("{b:0>8b}"));
            } else {
                println!("{}", String::from_utf8_lossy(&decoded))
            }
            std::io::stdout().flush()?;
        }
    }

    Ok(())
}
