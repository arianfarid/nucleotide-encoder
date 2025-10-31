use std::{convert::Infallible, fmt::{Debug, Display}, ops::Index};

use crate::base::{BaseMask, InvalidCodeError};

#[derive(Clone, Copy)]
pub struct BaseMaskWord(u8);
impl From<[Option<BaseMask>; 2]> for BaseMaskWord {
    fn from(value: [Option<BaseMask>; 2]) -> Self {
        BaseMaskWord(to_raw_word(value))
    }
}
fn to_raw_word(masks: [Option<BaseMask>; 2]) -> u8 {
    BaseMask::option_to_raw(masks[1]) << 4 | BaseMask::option_to_raw(masks[0])
}
fn from_raw_word(word: u8) -> [Option<BaseMask>; 2] {
    [
        BaseMask::from_raw(word & 0b1111),
        BaseMask::from_raw(word >> 4),
    ]
}
impl From<BaseMaskWord> for [Option<BaseMask>; 2] {
    fn from(value: BaseMaskWord) -> Self {
        from_raw_word(value.0)
    }
}
impl FromIterator<BaseMaskWord> for BaseMaskVec {
    fn from_iter<T: IntoIterator<Item = BaseMaskWord>>(iter: T) -> Self {
        BaseMaskVec(iter.into_iter().collect())
    }
}
impl BaseMaskWord {
    pub fn complement_rotate(self) -> Self {
        BaseMaskWord((self.0 & 0b00110011 << 2) | (self.0 & 0b00110011 >> 2))
    }
}

pub struct BaseMaskVec(Vec<BaseMaskWord>);

impl Index<usize> for BaseMaskVec {
    type Output = BaseMask;

    fn index(&self, index: usize) -> &Self::Output {
        <[Option<BaseMask>; 2]>::from(self.0[index / 2])[index % 2]
            .expect("index is out of bounds")
            .static_ref()
    }
}

#[derive(Clone, Copy)]
struct WordIterator<I> {
    iter: I,
    next_item: Option<BaseMask>,
}

impl<I> WordIterator<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            next_item: None,
        }
    }
}

impl<I: Iterator<Item = BaseMaskWord>> Iterator for WordIterator<I> {
    type Item = BaseMask;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_item {
            Some(mask) => {
                self.next_item = None;
                Some(mask)
            },
            None => {
                let items = <[Option<BaseMask>; 2]>::from(self.iter.next()?);
                self.next_item = items[1];
                items[0]
            },
        }
    }
}

pub struct WordIteratorVec(WordIterator<<Vec<BaseMaskWord> as IntoIterator>::IntoIter>);
impl Iterator for WordIteratorVec {
    type Item = BaseMask;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct WordIteratorSlice<'t>(
    WordIterator<std::iter::Copied<<&'t [BaseMaskWord] as IntoIterator>::IntoIter>>,
);
impl<'t> Iterator for WordIteratorSlice<'t> {
    type Item = BaseMask;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl IntoIterator for BaseMaskVec {
    type Item = BaseMask;

    type IntoIter = WordIteratorVec;

    fn into_iter(self) -> Self::IntoIter {
        WordIteratorVec(WordIterator::new(self.0.into_iter()))
    }
}

impl<'t> IntoIterator for &'t BaseMaskVec {
    type Item = BaseMask;

    type IntoIter = WordIteratorSlice<'t>;

    fn into_iter(self) -> Self::IntoIter {
        WordIteratorSlice(WordIterator::new(self.0.as_slice().into_iter().copied()))
    }
}

impl FromIterator<BaseMask> for BaseMaskVec {
    fn from_iter<T: IntoIterator<Item = BaseMask>>(iter: T) -> Self {
        let mut cached = None;
        let mut vec = Vec::<BaseMaskWord>::new();
        for item in iter {
            match cached {
                Some(_) => {
                    vec.push(BaseMaskWord::from([cached, Some(item)]));
                    cached = None;
                }
                None => {
                    cached = Some(item);
                }
            }
        }
        if cached.is_some() {
            vec.push(BaseMaskWord::from([cached, None]));
        }
        BaseMaskVec(vec)
    }
}

impl Display for BaseMaskVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for base in self {
            write!(f, "{}", base)?;
        }
        Ok(())
    }
}

impl Debug for BaseMaskVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl TryFrom<String> for BaseMaskVec {
    type Error = InvalidCodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
        .chars()
        .map(BaseMask::try_from)
        .collect()
        
    }
}

impl BaseMaskVec {
    pub fn complement_match(&self) -> Self {
        self.into_iter().map(|mask| mask.complement_match()).collect()
    }
    pub fn complement_rotate(&self) -> Self {
        // self.0.iter().map(|word| word.complement_rotate()).collect()
        self.into_iter().map(|mask| mask.complement_rotate()).collect()
    }
    pub fn complement_lookup(&self) -> Self {
        self.into_iter().map(|mask| mask.complement_lookup()).collect()
    }
}