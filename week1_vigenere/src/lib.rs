#![allow(unused_variables)]
#![allow(dead_code)]

struct Vigenere {
    key: String,
}

impl Vigenere {
    fn new(key: String) -> Vigenere {
        Vigenere { key }
    }

    fn generate_key(&self, text: String) -> String {
        let text_len = text.len();
        println!("text_len: {}", text_len);
        self.key
            .repeat(text_len / self.key.len() + 1)
            .chars()
            .take(text_len)
            .collect::<String>()
    }

    fn encrypt(&self, text: String) -> String {
        let a_byte = 'A' as u8 + 1;
        let text_len = text.len();
        let mut result = String::new();
        let text_bytes = text.as_bytes();
        let binding = self.generate_key(text.clone());
        let key_bytes = binding.as_bytes();
        for i in 0..text_len {
            let result_char = ((text_bytes[i] + key_bytes[i]) % 26 + a_byte) as char;
            result.push(result_char);
        }
        result
    }

    fn decrypt(&self, text: String) -> String {
        let a_byte = 'A' as u8 - 1;
        let text_len = text.len();
        let mut result = String::new();
        let text_bytes = text.as_bytes();
        let binding = self.generate_key(text.clone());
        let key_bytes = binding.as_bytes();
        for i in 0..text_len {
            let result_char = ((26 + text_bytes[i] - key_bytes[i]) % 26 + a_byte) as char;
            result.push(result_char);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let text = String::from("VINEGERE");
        let key = String::from("KEY");
        let vigenere = Vigenere::new(key);

        let cipher = vigenere.encrypt(text);
        let expected_cipher = String::from("GNMPLDCJ");
        assert_eq!(cipher, expected_cipher);

        let plain = vigenere.decrypt(cipher);
        let expected_plain = String::from("VINEGERE");
        assert_eq!(expected_plain, plain);
    }
}