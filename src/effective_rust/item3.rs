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
    }
}
