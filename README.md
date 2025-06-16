# DNA Compression in Pure Rust

This project provides a Rust implementation of a compact, reversible DNA compression format using 4-bit encoding. It supports all 15 IUPAC nucleotide codes, base-pair complement generation, and efficient serialization. The tool is intended for developers working with large-scale biological data or exploring performance-efficient data representations.

## Features

- 4-bit encoding of nucleotides (2× size reduction compared to plain text)
- Full IUPAC code support
- Bitwise base-pair complement generation
- Fast decompression with padding-aware decoding
- CLI interface with support for compression, decompression, complement, and benchmarking

---

## Installation

Clone the repository and build the tool using Cargo.

## Usage
Compress a DNA sequence
```bash
cargo run . -- input.txt
```
This will read input.txt (a plain-text DNA sequence) and write a compressed binary file to output.txt.

### Generate base-pair complements during compression
```bash
cargo run . -- input.txt --compliment
```
This applies a bitwise complement to the sequence prior to compression.
### Run performance benchmarks
```bash
cargo run . -- input.txt --benchmark
```
Runs a performance comparison between bitwise rotation and match-based complement logic. Benchmark output is written to speed_test.csv.

### Decompress a binary-encoded DNA file
```bash
cargo run . -- output.txt --decode
```
Reads output.txt (binary) and reconstructs the original DNA sequence in decoded.txt.
