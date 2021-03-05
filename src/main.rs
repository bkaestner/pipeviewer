use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::stdin();
    let mut output = std::io::stdout();
    let mut report = std::io::stderr();

    let mut buffer = [0; 1024];
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
