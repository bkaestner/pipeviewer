use anyhow::Result;
use humanize_rs::bytes::Bytes;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
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

    let (mut input, len): (Box<dyn Read>, Option<u64>) = if let Some(file) = opts.input {
        let file = File::open(file)?;
        let len = file.metadata()?.len();
        (Box::new(file), Some(len))
    } else {
        (Box::new(std::io::stdin()), None)
    };

    let pb = if let Some(len) = len {
        let pb = ProgressBar::new(len);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar} {bytes}/{total_bytes} ETA: {eta} {msg}"),
        );
        pb
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("[{elapsed_precise}] {spinner} {bytes} {msg}"),
        );
        pb
    };
    let mut output = pb.wrap_write(std::io::stdout());

    let mut buffer = vec![0; opts.buffer_size];

    while let Ok(n) = input.read(&mut buffer) {
        if n == 0 {
            break;
        }
        output.write_all(&buffer[..n])?;
    }
    Ok(())
}
