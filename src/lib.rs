use bitvec::prelude::*;

pub fn print_bits() {
    let mut buffer = "hello".as_bytes().to_owned();
    // Consume original buffer in u8 to a BitVec buffer
    let bits = buffer.view_bits::<Msb0>();
    let (head, rest) = bits.split_at(6);
    println!("{:?}", bits);
    println!("{:?}", head);
    println!("{:?}", rest);
    // for n in buffer.into_iter() {
    //     let mut bv: BitVec = n.into();
    //     bit_buff.append(&mut bv);
    // }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
