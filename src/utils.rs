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

#[inline]
pub fn gcd_u16(first: u16, second: u16) -> u16 {
    let mut a: u16 = first;
    let mut b: u16 = second;
    loop {
        if b == 0 {
            break;
        }

        let temp: u16 = b;
        b = a % b;
        a = temp;
    }
    
    a
}

#[inline]
pub fn gcd_u32(first: u32, second: u32) -> u32 {
    let mut a: u32 = first;
    let mut b: u32 = second;
    loop {
        if b == 0 {
            break;
        }

        let temp: u32 = b;
        b = a % b;
        a = temp;
    }
    
    a
}

#[inline]
pub fn gcd_u64(first: u64, second: u64) -> u64 {
    let mut a: u64 = first;
    let mut b: u64 = second;
    loop {
        if b == 0 {
            break;
        }

        let temp: u64 = b;
        b = a % b;
        a = temp;
    }
    
    a
}