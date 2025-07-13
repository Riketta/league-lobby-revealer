use std::char;

use anyhow::Result;

pub fn encode(input: &str) -> String {
    let mut encoded = String::new();

    for b in input.as_bytes() {
        if b.is_ascii_alphanumeric() || *b == b'-' || *b == b'.' || *b == b'_' || *b == b'~' {
            encoded.push(char::from_u32(u32::from(*b)).unwrap());
        } else if *b == b' ' {
            encoded.push('+');
        } else {
            encoded.push_str(&format!("%{b:02X}"));
        }
    }

    encoded
}

pub fn decode(input: &str) -> Result<String> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            match (chars.next(), chars.next()) {
                (Some(d1), Some(d2)) => {
                    if let (Some(hi), Some(lo)) = (d1.to_digit(16), d2.to_digit(16)) {
                        bytes.push(u8::try_from((hi << 4) | lo)?);
                    } else {
                        return Err(anyhow::Error::msg("invalid hex digits"));
                    }
                }
                _ => {
                    return Err(anyhow::Error::msg("incorrect percent ecndoed values"));
                }
            }
        } else if c == '+' {
            bytes.push(b' ');
        } else {
            bytes.push(u8::try_from(c)?);
        }
    }

    Ok(String::from_utf8(bytes)?)
}
