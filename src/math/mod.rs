// mod integer_galois;
// mod integer_galois_prime;
mod integer_unsafe;
mod galois_field;

pub use integer_unsafe::UnsafeInteger;

pub trait ShamirData: std::ops::MulAssign + std::ops::AddAssign + Sized {
    const CHUNK_BYTE_COUNT: usize;

    // Generative
    fn new() -> Self;
    fn new_int(a: u8) -> Self;
    fn new_fraction(a: i64, b: i64) -> Self;
    fn from_bytes(bytes: &[u8]) -> Self;
    fn get_random<R: rand::Rng>(rng: &mut R) -> Self;

    // Mutations
    fn mul(self, rhs: &Self) -> Self;
    fn pow(self, exp: u32) -> Self;

    // Getter
    fn get_data(&self) -> Vec<u8>;
}
