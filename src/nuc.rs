
#[derive(Debug)]
///Representation of a nucleotide pair.
pub struct NucWord(pub u16);
impl NucWord {
    pub fn from_str(nucleotides: &str) -> Self {
        let mut out: u16 = 0;
        for (i, nuc) in nucleotides.chars().enumerate() {
            let mask: u16 = match nuc {
                '_' => 0b0000,
                'A' => 0b0001,
                'C' => 0b0010,
                'T' => 0b0100,
                'G' => 0b1000,
                'R' => 0b0011,
                'K' => 0b0110,
                'Y' => 0b1100,
                'M' => 0b1001,
                'S' => 0b0101,
                'W' => 0b1010,
                'B' => 0b1110,
                'D' => 0b1101,
                'H' => 0b1011,
                'V' => 0b0111,
                'N' => 0b1111,
                _ => {
                    panic!("Invalid nucleotide {}", nuc);
                }
            };
            out |= (mask as u16) << (i * 4);
        }
        Self(out)
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for i in 0..4 {
            let bin = (self.0 >> (i * 4)) & 0b1111;
            let nuc = match bin {
                0b0000 => '_',
                0b0001 => 'A',
                0b0010 => 'C',
                0b0100 => 'T',
                0b1000 => 'G',
                0b0011 => 'R',
                0b0110 => 'K',
                0b1100 => 'Y',
                0b1001 => 'M',
                0b0101 => 'S',
                0b1010 => 'W',
                0b1110 => 'B',
                0b1101 => 'D',
                0b1011 => 'H',
                0b0111 => 'V',
                0b1111 => 'N',
                _ => {
                    panic!("Invalid nuc");
                }
            };
            // Step to filter out padded nucleotides
            if nuc == '_' {
                continue;
            }
            out.push(nuc);
        }
        out
    }
    pub fn compliment(&mut self, i: usize) {
        let shift = i * 4;
        let mask = 0b1111 << shift;
        let to_mask = (self.0 & mask) >> shift;
        let complement = (to_mask << 2 | to_mask >> 2) & 0b1111;
        self.0 = (self.0 & !mask) | (complement << shift);
    }
    pub fn compliment_match(&mut self, i: usize) {
        let shift = i * 4;
        let mask = 0b1111 << shift;
        let bin = (self.0 & mask) >> shift;

        let comp: u16 = match bin {
            0b0000 => 0b0000, // '_' → '_'
            0b0001 => 0b0100, // A → T
            0b0010 => 0b1000, // C → G
            0b0100 => 0b0001, // T → A
            0b1000 => 0b0010, // G → C

            0b0011 => 0b1100, // R (A or G) → Y (T or C)
            0b0110 => 0b1001, // K (G or T) → M (A or C)
            0b1100 => 0b0011, // Y (C or T) → R (G or A)
            0b1001 => 0b0110, // M (A or C) → K (T or G)

            0b0101 => 0b1010, // S (G or C) → W (A or T)
            0b1010 => 0b0101, // W (A or T) → S (C or G)

            0b1110 => 0b1011, // B (not A: C/G/T) → H (not G: A/C/T)
            0b1101 => 0b1101, // D (not C: A/G/T) → D (not C)
            0b1011 => 0b1110, // H (not G: A/C/T) → B (not A)
            0b0111 => 0b0111, // V (not T: A/C/G) → V (not T)

            0b1111 => 0b1111, // N → N
            _ => panic!("Invalid nucleotide bits: {:04b}", bin),
        };
        self.0 = (self.0 & !mask) | (comp << shift);
    }
    pub fn compliment_each(&mut self) {
        for i in 0..4 {
            self.compliment(i);
        }
    }
    pub fn compliment_each_match(&mut self) {
        for i in 0..4 {
            self.compliment_match(i);
        }
    }
}