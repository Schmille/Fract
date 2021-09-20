#[inline]
pub fn gcd_u8(first: u8, second: u8) -> u8 {
    let mut a: u8 = first;
    let mut b: u8 = second;
    loop {
        if b == 0 {
            break;
        }

        let temp: u8 = b;
        b = a % b;
        a = temp;
    }
    
    a
}