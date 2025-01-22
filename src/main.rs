fn main() {
    let dna = String::from("AGCTAGCT");
    // let b = b.bitand(rhs)
    println!("{:?}", dna);
    let dna_comp = QNuc::from_str(dna);
    println!("{:?}", dna_comp.to_string());
}

#[derive(Debug)]
pub struct QNuc(Vec<NucQuad>);
impl QNuc {
    pub fn from_str(nucleotides: String) -> Self {
        let mut out = QNuc(vec![]);
        for i in 0..(nucleotides.len() / 4 as usize) {
            let low = i * 4;
            let high = (i * 4) + 4;
            let str = &nucleotides[low..high];
            out.0.push(NucQuad::from_str(&str));
        }
        out
    }
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for quad in self.0.iter() {
            out.push_str(&quad.to_string());
        }
        out
    }
}

#[derive(Debug)]
///Representation of a nucleotide pair.
pub struct NucQuad(u32);
impl NucQuad {
    pub fn from_str(nuc_quad: &str) -> Self {
        let mut out: u32 = 0;
        for (i, nuc) in nuc_quad.chars().enumerate() {
            let bin = match nuc {
                'A' => 0b0000,
                'C' => 0b0001,
                'G' => 0b0010,
                'T' => 0b0011,
                '_' => 0b0100,
                'R' => 0b0101,
                'Y' => 0b0110,
                'S' => 0b0111,
                'W' => 0b1000,
                'K' => 0b1001,
                'M' => 0b1010,
                'B' => 0b1011,
                'D' => 0b1100,
                'H' => 0b1101,
                'V' => 0b1110,
                'N' => 0b1111,
                _ => {
                    panic!("Invalid nucleotide {}", nuc);
                }
            };
            out |= (bin as u32) << (i * 4);
        }
        Self(out)
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for i in 0..4 {
            let bin = (self.0 >> (i * 4)) & 0b1111;
            let nuc = match bin {
                0b0000 => 'A',
                0b0001 => 'C',
                0b0010 => 'G',
                0b0011 => 'T',
                0b0100 => '_',
                0b0101 => 'R',
                0b0110 => 'Y',
                0b0111 => 'S',
                0b1000 => 'W',
                0b1001 => 'K',
                0b1010 => 'M',
                0b1011 => 'B',
                0b1100 => 'D',
                0b1101 => 'H',
                0b1110 => 'V',
                0b1111 => 'N',
                _ => {
                    panic!("");
                }
            };
            out.push(nuc);
        }
        out
    }
}
