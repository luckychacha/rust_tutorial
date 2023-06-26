use regex::Regex;

struct InputData {
    payload: Option<Vec<u8>>,
}

impl InputData {
    fn encrypted(&self) -> Vec<u8> {
        // encrypt(&self.payload.unwrap_or(vec![]))

        // fn as_ref
        // Converts from `&Option<T>` to `Option<&T>`.
        // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
        encrypt(self.payload.as_ref().unwrap_or(&vec![]))
    }
}

fn encrypt(payload: &[u8]) -> Vec<u8> {
    payload.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt() {
        let i = InputData { payload: None };
        println!("i: {:?}", i.encrypted());

        // let unicode_string = "\u{534e}\u{90a6}\u{5065}\u{5eb7}\u{5fb7}\u{8c6a}";

        // let utf8_bytes: Vec<u8> = unicode_string
        //     .chars()
        //     .flat_map(|c| c.encode_utf8(&mut [0; 3]).bytes().collect::<Vec<_>>())
        //     .collect();

        // println!("{:?}", utf8_bytes);

        // let unicode_string = "\u{534e}\u{90a6}\u{5065}\u{5eb7} \u{5fb7}\u{8c6a}";

        // if let Ok(utf8_string) = String::from_utf8(utf8_bytes) {
        //     println!("{}", utf8_string);
        // } else {
        //     println!("Invalid UTF-8 sequence");
        // }

        let unicode_string = "\u{534e}\u{90a6}\u{5065}\u{5eb7}\u{5fb7}\u{8c6a}";

        let utf8_bytes = unicode_string.as_bytes();

        println!("&[u8]: {:?}", utf8_bytes);

        let utf8_string = String::from_utf8_lossy(utf8_bytes);

        println!("utf8_string: {}", utf8_string);

        // raw unicode
        let unicode_string = String::from(r#"ST\u534e\u90a6\u5065\u5eb7 "#);
        let re = Regex::new(r"\\u([0-9a-fA-F]{4})").unwrap();
        let replaced = re.replace_all(&unicode_string, |caps: &regex::Captures<'_>| {
            let hex = &caps[1];
            let num = u32::from_str_radix(hex, 16).unwrap();
            char::from_u32(num).unwrap().to_string()
        });

        println!("replaced: {replaced}");
    }
}
