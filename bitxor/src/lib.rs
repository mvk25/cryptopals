use num_bigint::{BigInt, ToBigInt};

pub fn bitxor(num_a: &[u8], num_b: &[u8]) -> Option<BigInt> {
    let a = BigInt::parse_bytes(num_a, 16);
    let b = BigInt::parse_bytes(num_b, 16);
    let c = a.unwrap() ^ b.unwrap();

    println!("{}", c);
    ToBigInt::to_bigint(&c)
}

mod tests {
    use super::*;

    #[test]
    fn test_bitxor() {
        let a = b"1c0111001f010100061a024b53535009181c";
        let b = b"686974207468652062756c6c277320657965";
        let c = b"746865206b696420646f6e277420706c6179";


        println!("{:?}", bitxor(a, b));

        assert_eq!(bitxor(a, b), BigInt::parse_bytes(c, 16));
    }
}
