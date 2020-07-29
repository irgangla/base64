
fn main() {
    println!("{}", base64_encode("Hello, world!"));
}

pub fn base64_encode(text: &str) -> String {
    let characters: &[u8] = text.as_bytes();
    let len = characters.len();
    let mut base64_output = vec![];

    let remaining = len % 3;

    for i in (0..len - remaining).step_by(3) {
        base64_output.append(&mut vec![
            bits_to_char(extract_first_character_bits(characters[i])),
            bits_to_char(extract_second_character_bits(characters[i], characters[i+1])),
            bits_to_char(extract_third_character_bits(characters[i+1], characters[i+2])),
            bits_to_char(extract_fourth_character_bits(characters[i+2])),
        ]);
    }

    if remaining == 1 {
        base64_output.append(&mut vec![
            bits_to_char(extract_first_character_bits(characters[len - 1])),
            bits_to_char(extract_second_character_bits(characters[len - 1], 0)),
            '=',
            '=',
        ]);
    } else if remaining == 2 {
        base64_output.append(&mut vec![
            bits_to_char(extract_first_character_bits(characters[len - 2])),
            bits_to_char(extract_second_character_bits(characters[len - 2], characters[len -1])),
            bits_to_char(extract_third_character_bits(characters[len - 1], 0)),
            '=',
        ]);
    }

    base64_output.into_iter().collect()
}

fn extract_first_character_bits(byte: u8) -> u8 {
    (byte & 0b11111100) >> 2
}

fn extract_second_character_bits(first_byte: u8, second_byte: u8) -> u8 {
    ((first_byte & 0b00000011) << 4) | ((second_byte & 0b11110000) >> 4)
}

fn extract_third_character_bits(second_byte: u8, third_byte: u8) -> u8 {
    ((second_byte & 0b00001111) << 2) | ((third_byte & 0b11000000) >> 6)
}

fn extract_fourth_character_bits(third_byte: u8) -> u8 {
    third_byte & 0b00111111
}

fn bits_to_char(bits: u8) -> char {
    println!("{}", bits);
    if bits <= 25 {
        return (('A' as u8) + bits) as char;
    } else if bits <= 51 {
        return (('a' as u8) + (bits - 26)) as char;
    } else if bits <= 61 {
        return (('0' as u8) + (bits - 52)) as char;
    } else if bits == 62 {
        return '+';
    } else {
        return '/';
    }
}

#[cfg(test)]
mod test {
    use super::base64_encode;

    #[test]
    fn encode_three_characters() {
        let content = "abc";
        let expected = "YWJj";

        assert_eq!(expected, base64_encode(content));
    }

    #[test]
    fn encode_six_characters() {
        let content = "ABCdef";
        let expected = "QUJDZGVm";

        assert_eq!(expected, base64_encode(content));
    }

    #[test]
    fn encode_four_characters() {
        let content = "rstu";
        let expected = "cnN0dQ==";

        assert_eq!(expected, base64_encode(content));
    }

    #[test]
    fn encode_five_characters() {
        let content = "efghi";
        let expected = "ZWZnaGk=";

        assert_eq!(expected, base64_encode(content));
    }
}