use anyhow::Result;
use humanize_rs::bytes::Bytes;
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

fn parse_bytes(src: &str) -> Result<usize, &'static str> {
    if let Ok(bytes) = src.parse::<Bytes>() {
        Ok(bytes.size())
    } else {
        Err("Invalid size format")
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "pipeviewer", about = "A pipe inspecting application.")]
struct Opt {
    /// Input file, stdin if not specified
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    /// Internal buffer size
    #[structopt(short, long, name = "SIZE", parse(try_from_str = parse_bytes), default_value = "2MiB")]
    buffer_size: usize,
}

fn main() -> Result<()> {
    let opts = Opt::from_args();

    let mut input: Box<dyn Read> = if let Some(file) = opts.input {
        Box::new(std::fs::File::open(file)?)
    } else {
        Box::new(std::io::stdin())
    };
    let mut output = std::io::stdout();
    let mut report = std::io::stderr();

    let mut buffer = vec![0; opts.buffer_size];
    let mut total = 0;

    while let Ok(n) = input.read(&mut buffer) {
        if n == 0 {
            break;
        }
        total += n;
        output.write_all(&buffer[..n])?;
        let _ = write!(report, "\r{}", total);
    }
    Ok(())
}
