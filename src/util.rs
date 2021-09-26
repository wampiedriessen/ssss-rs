use num_bigint::BigInt;

pub struct Fraction {
    pub num: BigInt,
    pub denum: BigInt,
}

impl Fraction {
    pub fn new(num: BigInt, denum: BigInt) -> Self {
        Fraction { num, denum }
    }
}

impl std::ops::AddAssign<Fraction> for Fraction {
    fn add_assign(&mut self, rhs: Fraction) {
        self.num = &rhs.denum * &self.num + &rhs.num * &self.denum;
        self.denum *= &rhs.denum;
    }
}

impl std::ops::MulAssign<Fraction> for Fraction {
    fn mul_assign(&mut self, rhs: Fraction) {
        self.num *= rhs.num;
        self.denum *= rhs.denum;
    }
}
