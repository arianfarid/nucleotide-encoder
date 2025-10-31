use nucleotide_encoder::baseword::BaseMaskVec;
use nucleotide_encoder::NucBlockVec;
use std::io::BufReader;
use std::time::Instant;
use std::{
    env,
    fs::File,
    io::{Read, Write},
};

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

    let mut vector = NucBlockVec::from_bytes(&compressed_data);

    if compliment {
        // vector.compliment_sequence_simd();
    }

    let decoded_dna = vector.to_string();
    File::create("decoded.txt")?.write_all(decoded_dna.as_bytes())?;
    println!("Decoded DNA written to decoded.txt");

    Ok(())
}

fn encode_flow(input_path: &str, compliment: bool, benchmark: bool) -> std::io::Result<()> {
    println!("Encoding {}", input_path);
    let mut dna = String::new();
    BufReader::new(File::open(input_path)?).read_to_string(&mut dna)?;

    if benchmark {
        run_benchmark_from_str(&mut dna)?;
    }
    let mut vector = match BaseMaskVec::try_from(dna) {
        Ok(v) => v,
        Err(_) => panic!("test"),
    };
    // println!("{}", vector);
    if benchmark {
        run_benchmark_compliment(&mut vector)?;
    }

    if compliment {
        // vector.complement_sequence();
    }

    // let compressed = vector.into();
    // File::create("output_2.txt")?.write_all(&compressed)?;
    // println!("Compressed DNA written to output_2.txt");

    Ok(())
}

fn run_benchmark_from_str(dna: &mut String ) -> std::io::Result<()> {
    let mut file = File::create("speed_encode_test.csv")?;
    writeln!(file, "method,iteration,time_ms")?;
    for i in 0..1000 {
        let start = Instant::now();
        NucBlockVec::from_str(dna.to_string());
        let elapsed = start.elapsed().as_nanos();
        writeln!(file, "from_str,{i},{elapsed}")?;
    }
    for i in 0..1000 {
        let start = Instant::now();
        NucBlockVec::from_str_simd(dna.to_string());
        let elapsed = start.elapsed().as_nanos();
        writeln!(file, "from_str,{i},{elapsed}")?;
    }

    println!("Benchmark written to speed_encode_test.csv");
    Ok(())
}

fn run_benchmark_compliment(vector: &mut BaseMaskVec) -> std::io::Result<()> {
    let mut file = File::create("speed_compliment_test_2.csv")?;
    writeln!(file, "method,iteration,time_ms")?;

    for i in 0..1000 {
        let start_fast = Instant::now();
        vector.complement_match();
        let elapsed_fast = start_fast.elapsed().as_nanos();
        writeln!(file, "match,{i},{elapsed_fast}")?;
    }
    for i in 0..1000 {
        let start_match = Instant::now();
        vector.complement_rotate();
        let elapsed_match = start_match.elapsed().as_nanos();
        writeln!(file, "rotate,{i},{elapsed_match}")?;
    }
    for i in 0..1000 {
        let start_match = Instant::now();
        vector.complement_lookup();
        let elapsed_match = start_match.elapsed().as_nanos();
        writeln!(file, "lookup,{i},{elapsed_match}")?;
    }

    println!("Benchmark written to speed_compliment_test_2.csv");
    Ok(())
}
