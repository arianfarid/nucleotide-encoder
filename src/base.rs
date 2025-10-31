use std::{fmt::{Debug, Display, Formatter, Result}, ops::{BitAnd, BitOr, Shl, Shr}, simd::{u8x1, Simd}};

#[derive(Copy, Clone)]
pub enum Base {
    A,
    C,
    T,
    G,
}

impl Base {
    pub fn complement_match(self) -> Self {
        match self {
            Self::A => Self::T,
            Self::C => Self::G,
            Self::T => Self::A,
            Self::G => Self::C,
        }
    }
}

impl Display for Base {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&char::from(*self), f)
    }
}

// impl Debug for Base {}

impl From<Base> for char {
    fn from(value: Base) -> Self {
        match value {
            Base::A => 'A',
            Base::C => 'C',
            Base::T => 'T',
            Base::G => 'G',
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum BaseMask {
    A = 0b0001,
    C = 0b0010,
    T = 0b0100,
    G = 0b1000,
    R = 0b0011,
    K = 0b0110,
    Y = 0b1100,
    M = 0b1001,
    S = 0b0101,
    W = 0b1010,
    B = 0b1110,
    D = 0b1101,
    H = 0b1011,
    V = 0b0111,
    N = 0b1111,
}

impl TryFrom<char> for BaseMask {
    type Error = InvalidCodeError;
    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'A' => Ok(BaseMask::A),
            'C' => Ok(BaseMask::C),
            'T' => Ok(BaseMask::T),
            'G' => Ok(BaseMask::G),
            'R' => Ok(BaseMask::R),
            'K' => Ok(BaseMask::K),
            'Y' => Ok(BaseMask::Y),
            'M' => Ok(BaseMask::M),
            'S' => Ok(BaseMask::S),
            'W' => Ok(BaseMask::W),
            'B' => Ok(BaseMask::B),
            'D' => Ok(BaseMask::D),
            'H' => Ok(BaseMask::H),
            'V' => Ok(BaseMask::V),
            'N' => Ok(BaseMask::N),
            _ => Err(InvalidCodeError),
        }
    }
}

impl From<BaseMask> for char {
    fn from(value: BaseMask) -> Self {
        match value {
            BaseMask::A => 'A',
            BaseMask::C => 'C',
            BaseMask::T => 'T',
            BaseMask::G => 'G',
            BaseMask::R => 'R',
            BaseMask::K => 'K',
            BaseMask::Y => 'Y',
            BaseMask::M => 'M',
            BaseMask::S => 'S',
            BaseMask::W => 'W',
            BaseMask::B => 'B',
            BaseMask::D => 'D',
            BaseMask::H => 'H',
            BaseMask::V => 'V',
            BaseMask::N => 'N',
        }
    }
}

impl Display for BaseMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&char::from(*self), f)
    }
}

impl Debug for BaseMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&char::from(*self), f)
    }
}

impl Shl<u8> for BaseMask {
    type Output = BaseMask;
    fn shl(self, rhs: u8) -> Self {
        let lhs = self as u8;
        let result = lhs << rhs;
        BaseMask::from_raw(result & 0b1111).unwrap_or(BaseMask::N)
    }
    
}

impl Shr<u8> for BaseMask {
    type Output = BaseMask;
    fn shr(self, rhs: u8) -> Self {
        let lhs = self as u8;
        let result = lhs >> rhs;
        BaseMask::from_raw(result & 0b1111).unwrap_or(BaseMask::N)
    }
    
}

impl BitOr for BaseMask {
    type Output = BaseMask;

    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs_bits = self as u8;
        let rhs_bits = rhs as u8;
        let result_bits = lhs_bits | rhs_bits;

        BaseMask::from_raw(result_bits)
            .unwrap()
    }
}

impl BitAnd for BaseMask {
    type Output = BaseMask;

    fn bitand(self, rhs: Self) -> Self::Output {
        let lhsbits = self as u8;
        let rhsbits = rhs as u8;
        let result = lhsbits & rhsbits;
        BaseMask::from_raw(result).unwrap()
    }
}

impl BitAnd<u8> for BaseMask {
    type Output = BaseMask;

    fn bitand(self, rhs: u8) -> Self::Output {
        BaseMask::from_raw((self as u8) & rhs).unwrap()
    }
}

impl BaseMask {
    pub fn complement_match(self) -> Self {
        match self {
            BaseMask::A => BaseMask::T,
            BaseMask::C => BaseMask::G,
            BaseMask::T => BaseMask::A,
            BaseMask::G => BaseMask::C,
            BaseMask::R => BaseMask::Y,
            BaseMask::K => BaseMask::M,
            BaseMask::Y => BaseMask::R,
            BaseMask::M => BaseMask::K,
            BaseMask::S => BaseMask::S,
            BaseMask::W => BaseMask::W,
            BaseMask::B => BaseMask::H,
            BaseMask::D => BaseMask::V,
            BaseMask::H => BaseMask::B,
            BaseMask::V => BaseMask::D,
            BaseMask::N => BaseMask::N,
        }
    }

    pub const fn complement_lookup(self) -> Self {
        LOOKUP[self as usize]
    }

    pub fn complement_rotate(self) -> Self {
        (self << 2 | self >> 2) & 0b1111
    }
    pub fn static_ref(self) -> &'static BaseMask {
        match self {
            BaseMask::A => &BaseMask::A,
            BaseMask::C => &BaseMask::C,
            BaseMask::T => &BaseMask::T,
            BaseMask::G => &BaseMask::G,
            BaseMask::R => &BaseMask::R,
            BaseMask::K => &BaseMask::K,
            BaseMask::Y => &BaseMask::Y,
            BaseMask::M => &BaseMask::M,
            BaseMask::S => &BaseMask::S,
            BaseMask::W => &BaseMask::W,
            BaseMask::B => &BaseMask::B,
            BaseMask::D => &BaseMask::D,
            BaseMask::H => &BaseMask::H,
            BaseMask::V => &BaseMask::V,
            BaseMask::N => &BaseMask::N,
        }
    }

    pub fn option_to_raw(mask: Option<Self>) -> u8 {
        mask.map_or(0, Self::to_raw)
    }

    pub fn to_raw(self) -> u8 {
        self as u8
    }
    pub fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            0b0000 => None,
            0b0001 => Some(BaseMask::A),
            0b0010 => Some(BaseMask::C),
            0b0100 => Some(BaseMask::T),
            0b1000 => Some(BaseMask::G),
            0b0011 => Some(BaseMask::R),
            0b0110 => Some(BaseMask::K),
            0b1100 => Some(BaseMask::Y),
            0b1001 => Some(BaseMask::M),
            0b0101 => Some(BaseMask::S),
            0b1010 => Some(BaseMask::W),
            0b1110 => Some(BaseMask::B),
            0b1101 => Some(BaseMask::D),
            0b1011 => Some(BaseMask::H),
            0b0111 => Some(BaseMask::V),
            0b1111 => Some(BaseMask::N),
            _ => panic!("Invalid Raw Mask Error {}", raw),
        }
    }
}

pub struct InvalidCodeError;


const LOOKUP: [BaseMask; 16] = [
    BaseMask::N,  // 0b0000 — invalid/placeholder
            BaseMask::T,  // 0b0001 — A
            BaseMask::G,  // 0b0010 — C
            BaseMask::Y,  // 0b0011 — R
            BaseMask::A,  // 0b0100 — T
            BaseMask::S,  // 0b0101 — S
            BaseMask::M,  // 0b0110 — K
            BaseMask::D,  // 0b0111 — V
            BaseMask::C,  // 0b1000 — G
            BaseMask::K,  // 0b1001 — M
            BaseMask::W,  // 0b1010 — W
            BaseMask::B,  // 0b1011 — H
            BaseMask::R,  // 0b1100 — Y
            BaseMask::V,  // 0b1101 — D
            BaseMask::H,  // 0b1110 — B
            BaseMask::N,  // 0b1111 — N
        ];