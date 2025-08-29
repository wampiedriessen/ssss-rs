pub type Result<T> = std::result::Result<T, SsssErr>;

#[derive(Debug, Clone)]
pub struct SsssErr;

impl std::fmt::Display for SsssErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "an error occured in ssss-rs")
    }
}