fn padding(key: &[u8]) -> Vec<u8> {
    let str_len = key.len();
    let ceil = ((str_len as f64) / 16.0).ceil();
    let padding = ((ceil * 16.0) - (str_len as f64)) as u8;
    let mut padd = key.to_vec();
    padd.extend(std::iter::repeat(padding as u8).take(padding as usize));

    padd
}


fn main() {
    println!("Hello, world!");
    println!("{:?}", String::from_utf8_lossy(&padding(b"Here we are at the apex of the dilemma")));
}
