const DEFAULT_ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub struct CodeGenerator {
    length: usize,
    alphabet: Vec<char>,
}

impl CodeGenerator {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            alphabet: DEFAULT_ALPHABET.chars().collect(),
        }
    }

    pub fn generate(&self) -> String {
        let length = self.length;
        nanoid::nanoid!(length, &self.alphabet)
    }
}
