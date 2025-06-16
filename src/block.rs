
use crate::nuc::NucWord;

#[derive(Debug)]
pub struct NucBlockVec(Vec<NucWord>);
impl NucBlockVec {
    pub fn from_str(nucleotides: String) -> Self {
        let mut out = NucBlockVec(vec![]);
        for i in 0..(nucleotides.len() / 4 as usize) {
            let low = i * 4;
            let high = (i * 4) + 4;
            let str = &nucleotides[low..high];
            out.0.push(NucWord::from_str(&str));
        }
        if nucleotides.len() % 4 != 0 {
            let low = nucleotides.len() - (nucleotides.len() % 4) as usize;
            let str = &nucleotides[low..nucleotides.len() as usize];
            out.0.push(NucWord::from_str(&str));
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
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter()
        .flat_map(|b| b.0.to_le_bytes())
        .collect()
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut out = NucBlockVec(vec![]);

        let size = std::mem::size_of::<u16>();
        for chunk in bytes.chunks(size) {
            if chunk.len() == size {
                let word = u16::from_le_bytes(chunk.try_into().unwrap()); 
                out.0.push(NucWord(word));
            }
        }
        out
    }
    pub fn complimentary_base_pair(&mut self, index: usize) {
        let i = index / 4;
        self.0[i].compliment(index % 4);
    }
    pub fn complement_sequence(&mut self) {
        for nuc_word in self.0.iter_mut() {
            nuc_word.compliment_each();
        }
    }
    pub fn complement_sequence_match(&mut self) {
        for nuc_word in self.0.iter_mut() {
            nuc_word.compliment_each_match();
        }
    }
}