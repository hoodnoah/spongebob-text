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

    #[test]
    fn test_convert_to_sb_text() {
        let str = "Hello, World!";
        let sb_text = crate::text_conversion::convert_to_sb_text(str);
        assert_eq!(sb_text, "HeLlO, wOrLd!");
    }
}
