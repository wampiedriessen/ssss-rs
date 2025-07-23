use std::convert::TryInto;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GF(u64);

pub const K: usize = 8;
pub const Q: u64 = 283;
static INVERSE_LUT: std::sync::OnceLock<Vec<GF>> = std::sync::OnceLock::new();

impl GF {
    pub fn new(val: u8) -> GF {
        // Sanity check!
        assert!((val as usize) < GF::number_of_elements());
        GF(val as u64)
    }

    pub fn number_of_elements() -> usize {
        let k: u32 = K.try_into().unwrap(); // Abort if the number doesn't fit in 32-bits
        let p_k = 2u64.checked_pow(k).unwrap(); // Abort if the number doesn't fit in 64-bits
        p_k as usize
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    fn get_inverse_lut() -> &'static [GF] {
        INVERSE_LUT.get_or_init(|| {
            // Build up the inverse table using brute force
            let mut lut = vec![GF(0); GF::number_of_elements()];

            // Find the inverse for each of the numbers {1, 2, ..., N-1}
            for x in 1..GF::number_of_elements() {
                // Scan the numbers {1, 2, ..., N-1} until we find the inverse
                let x = GF(x as u64);
                let mut found = false;
                for y in 1..GF::number_of_elements() {
                    let y = GF(y as u64);
                    if x * y == GF(1) {
                        lut[x.0 as usize] = y;
                        found = true;
                        break;
                    }
                }
                if !found {
                    unreachable!("Every non-zero number has an inverse");
                }
            }

            lut
        })
    }

    fn invert(self) -> Result<GF, String> {
        // Important: Zero has no inverse, it's invalid
        if self.0 == 0 {
            return Err("Zero has no inverse".to_string());
        }
        // Perform a lookup in the pre-computed table
        Ok(GF::get_inverse_lut()[self.0 as usize])
    }
}

impl Add<GF> for GF {
    type Output = GF;
    fn add(self, rhs: GF) -> GF {
        GF(self.0 ^ rhs.0)
    }
}

impl AddAssign<GF> for GF {
    fn add_assign(&mut self, rhs: GF) {
        self.0 ^= rhs.0;
    }
}

impl Neg for GF {
    type Output = GF;
    fn neg(self) -> GF {
        self
    }
}

impl Sub<GF> for GF {
    type Output = GF;
    fn sub(self, rhs: GF) -> GF {
        GF(self.0 ^ rhs.0)
    }
}

fn extract_bit(n: u64, i: usize) -> u64 {
    (n >> i) & 1
}

impl Mul<GF> for GF {
    type Output = GF;
    fn mul(self, rhs: GF) -> GF {
        // First we unpack to get the raw u64 and we implement the algorithm
        // directly over the bits, rather than using the field's add/sub operators.
        let mut a: u64 = self.0;
        let b: u64 = rhs.0;
        let mut c: u64 = 0;

        // Loop over each possible term
        for i in 0..K {
            if extract_bit(b, i) == 1 {
                c ^= a; // c = poly_add(c, a)
            }
            a <<= 1;
            if extract_bit(a, K) == 1 {
                a ^= Q; // a = poly_sub(a, Q)
            }
        }
        GF(c)
    }
}

impl MulAssign<GF> for GF {
    fn mul_assign(&mut self, rhs: GF) {
        self.0 = self.mul(rhs).value();
    }
}

impl Div<GF> for GF {
    type Output = GF;
    fn div(self, rhs: Self) -> GF {
        // Important: Cannot divide by zero
        if rhs.0 == 0 {
            panic!("Cannot divide by zero");
        }
        self * rhs.invert().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_numbers() {
        assert_eq!(GF::number_of_elements(), 2usize.pow(8));
        // value small enough, no panic
        GF::new(255);
    }

    #[test]
    fn add() {
        assert_eq!(GF::new(3), GF::add(GF::new(1), GF::new(2)));
        assert_eq!(GF::new(0), GF::add(GF::new(1), GF::new(1)));
        assert_eq!(GF::new(0), GF::add(GF::new(2), GF::new(2)));
        assert_eq!(GF::new(0), GF::add(GF::new(3), GF::new(3)));
        assert_eq!(GF::new(0), GF::add(GF::new(255), GF::new(255)));
    }
}