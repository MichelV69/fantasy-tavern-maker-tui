// ---- start of mod ----
pub mod tpp {

    pub fn trim_whitespace(s: String) -> String {
        let words: Vec<_> = s.split_whitespace().collect();
        words.join(" ")
    }

    pub fn enum_string_to_phrase(s: String) -> String {
        let mut result = "".to_string();
        for c in s.chars() {
            result = if c.to_string() == c.to_lowercase().to_string() {
                format!("{}{}", result, c)
            } else {
                format!("{} {}", result, c.to_lowercase())
            };
        }
        result = result.replace("- ", "-");
        result.trim().to_owned()
    }

    pub fn tidy(s: String) -> String {
        trim_whitespace(enum_string_to_phrase(s))
    }

    pub fn l1_heading(s: String) -> String {
        let mut underline_len: usize = s.len();
        if underline_len > 42 {
            underline_len = 42
        }
        let underline = "=".repeat(underline_len);
        format!("\n\n{}\n{}\n\n", s, underline).trim().to_string()
    }

    pub fn l2_heading(s: String) -> String {
        let mut underline_len: usize = s.len();
        if underline_len > 42 {
            underline_len = 42
        }
        let underline = "-".repeat(underline_len);
        format!("\n{}\n{}\n", s, underline).trim().to_string()
    }

    pub fn l3_heading(s: String) -> String {
        format!("\n==={}===\n", s).trim().to_string()
    }

    pub fn is_a_an(test: &str) -> String {
        use ::is_vowel::IsRomanceVowel;
        if test
            .chars()
            .nth(0)
            .expect("Expected string input")
            .is_romance_vowel()
        {
            return "an".to_string();
        } else {
            return "a".to_string();
        };
    }
}
// ---- end of mod ----
