use std::convert::TryInto;

#[inline]
pub fn f32_from_str(_src: &str) -> f32 {
    let x = Unpacked::new(5, 6);
    let bits = (x.k as u64) << 23 | x.sig;
    f32::from_bits(bits.try_into().unwrap_or_else(|_| unreachable!()))
}

struct Unpacked {
    pub sig: u64,
    pub k: i16,
}

impl Unpacked {
    pub fn new(sig: u64, k: i16) -> Self {
        Unpacked { sig, k }
    }
}



