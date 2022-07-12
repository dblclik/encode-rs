use bitvec::prelude::*;

enum B64Variant {
    Standard,
    URLSafe,
    Radix64,
    IMAP,
    UTF7,
    MIME,
}

impl Default for B64Variant {
    fn default() -> Self {
        B64Variant::Standard
    }
}

struct B64Map {
    encoding_map: Vec<char>,
    variant: B64Variant,
}

impl B64Map {
    fn new(variant: B64Variant) -> B64Map {
        let b64_vec = match variant {
            B64Variant::Standard => {
                vec![
                    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                    'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
                ]
            }
            _ => {
                vec![]
            }
        };
        let b64_map = B64Map {
            encoding_map: b64_vec,
            variant,
        };
        b64_map
    }
}

pub fn b2(bits: &bitvec::slice::BitSlice<u8, bitvec::order::Msb0>) -> u32 {
    let base: u32 = 2;
    let mut value = 0;
    let mut index: u32 = bits.len() as u32;
    let value = bits.iter().fold(0, |_, x| {
        index -= 1;
        if *x {
            value = value + base.pow(index);
        }
        value
    });
    value
}

pub fn print_bits() {
    let buffer = "hello this is a much longer test and I am curious about what causes this issue"
        .as_bytes()
        .to_owned();
    // Consume original buffer in u8 to a BitVec buffer
    let bits = buffer.view_bits::<Msb0>();
    // println!("{:?}", bits);
    let bits_iter = bits.chunks(6);
    let b64_map = B64Map::new(B64Variant::default());
    let mut encoded_string = String::from("");
    for chunk in bits_iter {
        let load_u8 = chunk.load_be::<u8>();
        encoded_string.push(b64_map.encoding_map[load_u8 as usize]);
    }
    println!("{}", encoded_string);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
