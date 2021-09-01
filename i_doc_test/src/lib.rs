// Remove warnings generated with: `cargo build` and `cargo doc`.
// Implement stubs for `test_sub` and `test_bad_sub`.
// Add `test_add`, which tests the correct result of `operations::test_add,`
// and `test_add_overflow`, which generates overflow (check panic message to
// know what to expect in should_panic).

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

pub fn add_one(n: u8) -> u8 {
    n + 1
}

pub mod operations {
    pub fn add(a: u8, b: u8) -> u8 {
        a + b
    }

    pub fn sub(a: u32, b: u32) -> u32 {
        if b > a {
            panic!("b cannot be bigger than a");
        }

        b - a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        // Implement
    }

    #[test]
    #[should_panic(expected = "b cannot be bigger than a")]
    fn test_bad_sub() {
        // Implement
    }

    // Add `test_add` and `test_add_overflow`
}
