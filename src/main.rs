use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;
            let mut header = [0; 100];
            file.read_exact(&mut header)?;
            println!("Header: \n{}", String::from_utf8_lossy(&header));

            let page_size = u16::from_be_bytes([header[16], header[17]]);
            println!("database page size: {}", page_size); // 4096 bytes long

            let mut body = vec![0; (page_size - 100).into()];
            file.read_exact(&mut body)?;
            println!(
                "number of tables: {}",
                String::from_utf8_lossy(&body)
                    .matches("CREATE TABLE")
                    .count()
                    - String::from_utf8_lossy(&body)
                        .matches("CREATE TABLE sqlite_")
                        .count()
            );
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
