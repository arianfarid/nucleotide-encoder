use std::{env, fs::File, io::{Read, Write}};
use std::io::BufReader;
use std::time::Instant;
use rust_dna_compression::{ NucBlockVec };

fn main() -> std::io::Result<()> {
    // Read CLI args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let input_path = &args[1];
    let decode = args.contains(&"--decode".to_string());
    let compliment = args.contains(&"--compliment".to_string());
    let run_benchmark = args.contains(&"--benchmark".to_string());

    if decode {
        decode_flow(input_path, compliment)?;
    } else {
        encode_flow(input_path, compliment, run_benchmark)?;
    }

    Ok(())
}


fn decode_flow(input_path: &str, compliment: bool) -> std::io::Result<()> {
    println!("Decoding {}", input_path);
    let mut compressed_data = Vec::new();
    BufReader::new(File::open(input_path)?).read_to_end(&mut compressed_data)?;

    let mut dnablocks = NucBlockVec::from_bytes(&compressed_data);

    if compliment {
        dnablocks.complement_sequence();
    }

    let decoded_dna = dnablocks.to_string();
    File::create("decoded.txt")?.write_all(decoded_dna.as_bytes())?;
    println!("Decoded DNA written to decoded.txt");

    Ok(())
}

fn encode_flow(input_path: &str, compliment: bool, benchmark: bool) -> std::io::Result<()> {
    println!("Encoding {}", input_path);
    let mut dna = String::new();
    BufReader::new(File::open(input_path)?).read_to_string(&mut dna)?;

    let mut dnablocks = NucBlockVec::from_str(dna);

    if benchmark {
        run_benchmark(&mut dnablocks)?;
    }

    if compliment {
        dnablocks.complement_sequence();
    }

    let compressed = dnablocks.to_bytes();
    File::create("output.txt")?.write_all(&compressed)?;
    println!("Compressed DNA written to output.txt");

    Ok(())
}

fn run_benchmark(blocks: &mut NucBlockVec) -> std::io::Result<()> {
    let mut file = File::create("speed_test.csv")?;
    writeln!(file, "method,iteration,time_ms")?;

    for i in 0..100 {
        let start_fast = Instant::now();
        blocks.complement_sequence();
        let elapsed_fast = start_fast.elapsed().as_millis();
        writeln!(file, "bit_shift,{i},{elapsed_fast}")?;

        let start_match = Instant::now();
        blocks.complement_sequence_match();
        let elapsed_match = start_match.elapsed().as_millis();
        writeln!(file, "match_table,{i},{elapsed_match}")?;
    }

    println!("Benchmark written to speed_test.csv");
    Ok(())
}