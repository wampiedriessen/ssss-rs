struct GaloisFieldShamir {
    data: Vec<GaloisFieldFraction>
}

struct GaloisFieldFraction {
    num: u8,
    denum: u8
}

const RIJNDAEL_MOD: u16 = 0b1_0001_1011;
const SHORT_RIJNDAEL_MOD: u8 = (RIJNDAEL_MOD % 256) as u8;

fn gf_add(a: u8, b: u8) -> u8 {
    a ^ b
}

fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    // From the wiki:
    let mut p = 0u8;
    // Run the following loop eight times (once per bit). It is OK to stop when a or b is zero before an iteration:
    for _ in 0..8u8 {
        // 1. If the rightmost bit of b is set, exclusive OR the product p by the value of a. This is polynomial addition.
        // [WD] b-rightmost is set? xor with (127 & a) = a. not? xor with (0 & a) = 0.
        let (mask, _) = 0u8.overflowing_sub(b & 1);
        p ^= mask & a;
        // 2. Shift b one bit to the right, discarding the rightmost bit, and making the leftmost bit have a value of zero. This divides the polynomial by x, discarding the x0 term.
        b >>= 1;
        // 3. Keep track of whether the leftmost bit of a is set to one and call this value carry.
        let (mask, _) = 0u8.overflowing_sub((a >> 7) & 1); // [WD] 127 if there was a carry, zero if not
        // 4. Shift a one bit to the left, discarding the leftmost bit, and making the new rightmost bit zero. This multiplies the polynomial by x, but we still need to take account of carry which represented the coefficient of x7.
        a <<= 1;
        // 5. If carry had a value of one, exclusive or a with the hexadecimal number 0x1b (00011011 in binary). 0x1b corresponds to the irreducible polynomial with the high term eliminated. Conceptually, the high term of the irreducible polynomial and carry add modulo 2 to 0.
        a ^= SHORT_RIJNDAEL_MOD & mask;
    }

    p
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase {
        a: u8,
        b: u8,
        sum: u8,
        mul: u8
    }

    impl TestCase {
        const fn new(a: u8, b: u8, sum: u8, mul: u8) -> Self {
            TestCase { a, b, sum, mul }
        }
    }

    const TESTCASES: [TestCase; 5] = [
        TestCase::new(11, 12, 7, 116),
        TestCase::new(20, 68, 80, 39),
        TestCase::new(3, 5, 6, 15),
        TestCase::new(6, 6, 0, 20),
        TestCase::new(0x53, 0xCA, 0x99, 0x1),
    ];

    #[test]
    fn test_addition() {
        for case in TESTCASES {

            assert_eq!(gf_add(case.a, case.b), case.sum);
        }
    }

    #[test]
    fn test_multiplication() {
        for case in TESTCASES {
            assert_eq!(gf_mul(case.a, case.b), case.mul);
        }
    }
}
