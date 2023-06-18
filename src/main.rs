use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, Result};
use std::io::BufReader;
use std::time::Instant;

fn main() -> Result<()> {
    if args().len() !=3{
        eprint!("Usage : 'source' 'target' ");
        return Ok(())
    }

    let input_path = args().nth(1).expect("Input file path is required");
    let input_file = File::open(&input_path)?;
    let mut input = BufReader::new(input_file);

    let output_path = args().nth(2).expect("Output file path is required");
    let output_file = File::create(&output_path)?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder)?;
    let output = encoder.finish()?;

    let input_length = input.get_ref().metadata()?.len();
    let output_length = output.metadata()?.len();

    println!("Source Length: {:?}", input_length);
    println!("Target Length: {:?}", output_length);
    println!("Time Elapsed: {:?}", start.elapsed());

    Ok(())
}
