#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub use_commas: bool,
    pub use_and: bool,
    pub append_only: bool,
    pub uppercase: bool,
    pub capitalize: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            use_commas: false,
            use_and: false,
            append_only: false,
            uppercase: false,
            capitalize: false,
        }
    }
}
