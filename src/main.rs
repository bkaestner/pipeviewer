use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pipeviewer", about = "A pipe inspecting application.")]
struct Opt {
    // TODO: Handle more input files
    /// Input file, stdin if not specified
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Opt::from_args();

    let (mut input, len): (Box<dyn Read>, Option<u64>) = if let Some(file) = opts.input {
        let file = File::open(file)?;
        let len = file.metadata()?.len();
        (Box::new(file), Some(len))
    } else {
        (Box::new(io::stdin()), None)
    };

    let pb = if let Some(len) = len {
        let pb = ProgressBar::new(len);
        pb.set_style(ProgressStyle::default_bar().template(
            "[{elapsed_precise}] {bar} {bytes_per_sec} [{bytes}/{total_bytes}] ETA: {eta}",
        ));
        pb
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("[{elapsed_precise}] {spinner} {bytes_per_sec} [{bytes}]"),
        );
        pb
    };
    let mut output = pb.wrap_write(io::stdout());

    match io::copy(&mut input, &mut output) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == ErrorKind::BrokenPipe => Ok(()),
        Err(e) => Err(e)?,
    }
}
