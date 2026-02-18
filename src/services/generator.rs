pub struct CodeGenerator {
    pub length: usize,
    pub alphabet: Vec<char>,
}

impl CodeGenerator {
    pub fn new(length: usize, alphabet_str: &str) -> Self {
        let alphabet_vec: Vec<char> = alphabet_str.chars().collect();

        Self {
            length,
            alphabet: alphabet_vec,
        }
    }

    pub fn generate(&self) -> String {
        let length = self.length;
        nanoid::nanoid!(length, &self.alphabet)
    }
}
