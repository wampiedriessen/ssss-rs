use std::ops::{AddAssign, MulAssign};
use crate::math::GF;

pub struct GfPoly {
    data: Vec<GF>,
}

impl GfPoly {
    pub fn new(data: &Vec<u8>) -> GfPoly {
        GfPoly {
            data: data.clone()
                .into_iter()
                .map(GF::new)
                .collect()
        }
    }

    pub fn apply_x(&self, x: u8) -> GF {
        let mut val = GF::new(0);

        for i in 0..self.data.len() {
            let mut term = self.data[i].clone();
            for _ in 0..i {
                term *= GF::new(x);
            }
            val += term;
        }

        val
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_apply_x() {
        // 5 + x + 3x^2
        let poly: GfPoly = GfPoly::new(&vec![5u8, 1u8, 3u8]);

        assert_eq!(GF::new(5) + GF::new(3) + (GF::new(3) * GF::new(3) * GF::new(3)), poly.apply_x(3));
        assert_eq!(GF::new(5) + GF::new(4) + (GF::new(3) * GF::new(4) * GF::new(4)), poly.apply_x(4));
        assert_eq!(GF::new(5) + GF::new(5) + (GF::new(3) * GF::new(5) * GF::new(5)), poly.apply_x(5));
    }
}