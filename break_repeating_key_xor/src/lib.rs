pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    let mut difference: u32 = 0;
    for i in 0..a.len() {
        let k = format!("{:08b} {:08b}", a[i], b[i]);
        let m = k.split(' ').collect::<Vec<&str>>();
        let m_len = m[0].len();
        let (u, v) = (m[0], m[1]);
        for i in 0..m_len {
            if u.chars().nth(i) != v.chars().nth(i) {
                difference += 1;
            }
        }
    }
    
    difference
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_dist() {
        let a = "this is a test".as_bytes();
        let b = "wokka wokka!!!".as_bytes();
        assert_eq!(hamming_distance(a, b), 37);
    }
}


