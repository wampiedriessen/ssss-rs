use num_bigint::{BigInt, RandomBits, Sign};

use super::ShamirInteger;

pub struct UnsafeInteger {
    num: BigInt,
    denum: BigInt,
}

impl ShamirInteger for UnsafeInteger {
    fn new() -> Self {
        UnsafeInteger {
            num: 0.into(),
            denum: 1.into(),
        }
    }

    fn new_int(a: u8) -> Self {
        UnsafeInteger {
            num: a.into(),
            denum: 1.into(),
        }
    }

    fn new_fraction(a: i64, b: i64) -> Self {
        UnsafeInteger {
            num: a.into(),
            denum: b.into(),
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        UnsafeInteger {
            num: BigInt::from_bytes_be(Sign::Plus, bytes),
            denum: 1.into(),
        }
    }

    fn get_random<R: rand::Rng>(rng: &mut R, num_bits: u64) -> Self {
        UnsafeInteger {
            num: rng.sample(RandomBits::new(num_bits)),
            denum: 1.into(),
        }
    }

    fn mul(self, rhs: &UnsafeInteger) -> UnsafeInteger {
        UnsafeInteger {
            num: self.num * rhs.num.clone(),
            denum: self.denum * rhs.denum.clone(),
        }
    }

    fn pow(self, exp: u32) -> Self {
        if exp == 0 {
            return UnsafeInteger::new_int(1);
        }
        if exp == 1 {
            return self;
        }
        let mut f = UnsafeInteger {
            num: self.num.clone(),
            denum: self.denum.clone(),
        };

        for _ in 0..(exp - 1) {
            f = f.mul(&self);
        }

        f
    }

    fn get_data(&self) -> Vec<u8> {
        let (_, bytes) = self.normalize().num.to_bytes_be();

        bytes
    }
}

impl std::ops::MulAssign for UnsafeInteger {
    fn mul_assign(&mut self, rhs: UnsafeInteger) {
        self.num *= rhs.num;
        self.denum *= rhs.denum;
    }
}

impl std::ops::AddAssign for UnsafeInteger {
    fn add_assign(&mut self, rhs: UnsafeInteger) {
        self.num = &rhs.denum * &self.num + &rhs.num * &self.denum;
        self.denum *= &rhs.denum;
    }
}

impl UnsafeInteger {
    fn normalize(&self) -> Self {
        UnsafeInteger {
            num: self.num.clone() / self.denum.clone(),
            denum: 1.into(),
        }
    }
}
