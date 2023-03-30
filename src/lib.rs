pub mod text_conversion {
    /// Converts a string to SpongeBob text.
    ///
    /// # Examples
    /// ```
    /// use sb::text_conversion;
    /// let input_text = "Hello, World!";
    /// let output_text = text_conversion::convert_to_sb_text(input_text);
    /// assert_eq!(output_text, "HeLlO, wOrLd!");
    /// ```
    pub fn convert_to_sb_text(str: &str) -> String {
        let mut output_str = String::new();

        let mut is_upper = true; // start w/ uppercase

        for char in str.chars() {
            if char.is_alphabetic() {
                if is_upper {
                    output_str.push(char.to_uppercase().next().unwrap());
                    is_upper = !is_upper;
                } else {
                    output_str.push(char.to_lowercase().next().unwrap());
                    is_upper = !is_upper;
                }
            } else {
                output_str.push(char);
            }
        }

        output_str
    }
}

pub mod arg_parsing {
    /// Reads the first argument from the command line into a string.
    pub fn read_args(mut args_iter: impl Iterator<Item = String>) -> Result<String, &'static str> {
        args_iter.next(); // ignore first arg

        match args_iter.next() {
            Some(arg) => Ok(arg),
            None => Err("Did not pass in a string to be converted"),
        }
    }
}

#[cfg(test)]
mod sb_tests {
    // testing libraries
    use proptest::prelude::*;

    // module(s) under test
    use crate::text_conversion::convert_to_sb_text;

    const INPUT_REGEX: &'static str = "[a-zA-Z0-9[[:punct:]] ]+";

    /// Counts the number of capital letters in a string.
    fn count_capital_letters(in_string: &str) -> usize {
        in_string.chars().filter(|ch| ch.is_uppercase()).count()
    }

    /// Returns an iterator over pairs of adjacent elements in the slice.
    fn pairwise<T>(items: &[T]) -> impl Iterator<Item = (&T, &T)> {
        items.iter().zip(items.iter().skip(1))
    }

    #[test]
    fn test_convert_to_sb_text() {
        let str = "Hello, World!";
        let sb_text = convert_to_sb_text(str);
        assert_eq!(sb_text, "HeLlO, wOrLd!");
    }

    proptest! {
        #[test]
        fn is_idempotent(input_string in INPUT_REGEX) {
            let result1 = convert_to_sb_text(&input_string);
            let result2 = convert_to_sb_text(&input_string);

            assert_eq!(result1, result2);
        }
    }

    proptest! {
        #[test]
        fn result_same_length_as_input(input_string in INPUT_REGEX) {
            let result = convert_to_sb_text(&input_string);
            assert_eq!(result.len(), input_string.len());
        }
    }

    proptest! {
        #[test]
        fn same_num_caps_in_reverse(input_string in INPUT_REGEX) {
            let reverse_string: String = input_string.chars().rev().collect();

            let forward_result = convert_to_sb_text(&input_string);
            let reverse_result = convert_to_sb_text(&reverse_string);

            assert_eq!(count_capital_letters(&forward_result), count_capital_letters(&reverse_result));
        }
    }

    proptest! {
        #[test]
        fn alternating_caps(input_string in INPUT_REGEX) {
            let result = convert_to_sb_text(&input_string);

            let alphabet_chars = result.chars().filter(|ch| ch.is_alphabetic()).collect::<Vec<char>>();

            for (ch1, ch2) in pairwise(&alphabet_chars) {
                assert_ne!(ch1.is_uppercase(), ch2.is_uppercase());
            }
        }
    }
}
