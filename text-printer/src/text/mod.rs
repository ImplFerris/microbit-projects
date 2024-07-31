use heapless::Vec;

use crate::text::char_map::{CAP_LETTER_MATRICES, DIGIT_MATRICES, SMALL_LETTER_MATRICES};

pub mod char_map;

pub struct Text {
    pub chars: Vec<char_map::LedMatrix, 50>,
}

#[derive(Debug)]
pub enum TextError {
    VecError,
    InvalidChar,
}

#[allow(non_contiguous_range_endpoints)]
impl TryFrom<&str> for Text {
    type Error = TextError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = Vec::new();
        for c in value.chars() {
            let ascii = c as u8;
            match ascii {
                0x30..0x40 => {
                    let index = ascii as usize - 0x30;
                    chars
                        .push(DIGIT_MATRICES[index])
                        .map_err(|_| TextError::VecError)?;
                }
                0x41..0x5B => {
                    let index = ascii as usize - 0x41;
                    chars
                        .push(CAP_LETTER_MATRICES[index])
                        .map_err(|_| TextError::VecError)?;
                }
                0x61..0x7B => {
                    let index = ascii as usize - 0x61;
                    chars
                        .push(SMALL_LETTER_MATRICES[index])
                        .map_err(|_| TextError::VecError)?;
                }
                _ => return Err(TextError::InvalidChar),
            }
        }

        Ok(Self { chars })
    }
}
