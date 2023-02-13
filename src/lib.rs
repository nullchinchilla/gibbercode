use std::{collections::HashMap, num::Wrapping};

use itertools::Itertools;

fn encode_with_alphabet(mut to_code: u128, alphabet: &[u8]) -> String {
    let base = alphabet.len() as u128;
    let mut accum = Vec::new();
    while to_code > 0 {
        accum.push(alphabet[(to_code % base) as usize]);
        to_code /= base
    }
    String::from_utf8_lossy(&accum).to_string()
}

fn decode_with_alphabet(to_decode: &[u8], alphabet: &[u8]) -> u128 {
    let mapping: HashMap<u8, u128> = alphabet
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i as u128))
        .collect();
    to_decode
        .iter()
        .rev()
        .fold(Wrapping(0), |accum, c| {
            accum * Wrapping(alphabet.len() as u128) + Wrapping(mapping[c])
        })
        .0
}

const VOWELS: &[u8] = b"aiouy";
const CONSONANTS: &[u8] = b"kgsztdnpbmjrlwvxq";

/// Encodes two numbers with gibbercode. We take u128 because almost any number can fit in it.
pub fn encode(major: u128, minor: u128) -> String {
    let consonants = encode_with_alphabet(major, CONSONANTS);
    let vowels = encode_with_alphabet(minor, VOWELS);
    let consonants_padded = consonants.chars().chain(std::iter::repeat('h'));
    let vowels_padded = vowels.chars().chain(std::iter::repeat('e'));
    consonants_padded
        .chunks(2)
        .into_iter()
        .zip(vowels_padded)
        .map(|(c, v)| {
            let c = c.collect_vec();
            format!("{}{}{}", c[0], v, c[1])
        })
        .take_while(|syllable| syllable != "heh")
        .chunks(2)
        .into_iter()
        .map(|mut c| c.join("").replace("hh", ""))
        .join("-")
}

/// Decodes a gibbercode string into two numbers. This is actually infallible, because it simply ignores any garbage in the input and only decodes the part that looks like a gibbercode.
pub fn decode(gibber: &str) -> (u128, u128) {
    let gibber = gibber.as_bytes();
    let vowels = gibber
        .iter()
        .filter(|c| VOWELS.iter().contains(c))
        .copied()
        .collect_vec();
    let consonants = gibber
        .iter()
        .filter(|c| CONSONANTS.iter().contains(c))
        .copied()
        .collect_vec();
    (
        decode_with_alphabet(&consonants, CONSONANTS),
        decode_with_alphabet(&vowels, VOWELS),
    )
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode, encode_with_alphabet};

    #[test]
    fn encode_trivial() {
        let alphabet = b"abcdefghijklmnop";
        assert_eq!(encode_with_alphabet(31415926, alphabet), "ghofpnb");
    }

    #[test]
    fn encode_full() {
        assert_eq!(encode(23242151, 123), "nurlyt-nyq");
    }

    #[test]
    fn decode_full() {
        assert_eq!(decode("nurlyt-nyq"), (23242151, 123));
    }
}
