use std::{fs::File, io::{Read, Write}};
use std::io::BufReader;
use std::time::Instant;
fn main() -> std::io::Result<()> {

    // 1. Read raw DNA text
    let mut dna = String::new();
    BufReader::new(File::open("AE014297.txt")?).read_to_string(&mut dna)?;

    // 2. Compress into binary
    let mut dnablocks = NucBlockVec::from_str(dna);

    let mut file = File::create("speed_test3.csv")?;
    writeln!(file, "method,iteration,time_ns")?;

    for i in 0..100 {
        let start_fast = Instant::now();
        dnablocks.complement_sequence();
        let elapsed_fast = start_fast.elapsed().as_millis();
        writeln!(file, "bit_shift,{i},{elapsed_fast}")?;

        let start_match = Instant::now();
        dnablocks.complement_sequence_match();
        let elapsed_match = start_match.elapsed().as_millis();
        writeln!(file, "match_table,{i},{elapsed_match}")?;
    }

    let compressed = dnablocks.to_bytes();

    // 3. Write compressed data
    File::create("output.txt")?.write_all(&compressed)?;
    
    Ok(())
}