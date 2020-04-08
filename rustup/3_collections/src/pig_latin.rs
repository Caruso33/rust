use std::io::stdin;

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyString,
}

/// Return true if character is a vowel
/// ```
/// assert_eq!(is_vowel('a'), true);
/// assert_eq!(is_vowel('f'), false);
/// ```
fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

/// Return None on zero length string.
pub fn pig_latin(inp: &str) -> Result<String, Error> {
    let mut it = inp.chars();
    let first = it.next().ok_or(Error::EmptyString)?;

    if it.next() == None {
        Ok(format!("{}ay", first))
    } else if is_vowel(&first) {
        Ok(format!("{}-hay", inp))
    } else {
        Ok(format!("{}-{}ay", &inp[first.len_utf8()..], first))
    }
}

pub fn main() {
    // loop till `exit` found
    loop {
        let input = {
            let mut buf = String::new();
            stdin().read_line(&mut buf).expect("Failed to read line");
            buf.trim().to_string()
        };
        if input.to_lowercase() == "exit" {
            break;
        } else {
            match pig_latin(&input) {
                Ok(out) => println!("{}", out),
                Err(Error::EmptyString) => println!("ZERO LENGTH STRINGS NOT ALLOWED"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pig_latin() {
        assert_eq!(pig_latin("apple").unwrap(), "apple-hay".to_string());
        assert_eq!(pig_latin("first").unwrap(), "irst-fay".to_string());
    }

    #[test]
    fn test_pig_latin_single_char() {
        assert_eq!(pig_latin("f").unwrap(), "fay".to_string());
        assert_eq!(pig_latin("क").unwrap(), "कay".to_string());
    }

    #[test]
    fn test_pig_latin_unicode() {
        assert_eq!(pig_latin("नमस्कार").unwrap(), "मस्कार-नay".to_string());
    }

    #[test]
    fn test_pig_latin_zero_length() {
        assert_eq!(pig_latin("").unwrap_err(), Error::EmptyString);
    }
}
