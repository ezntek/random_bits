#[cfg(test)]
mod tests {
    use crate::check_consecutive_letters;

    #[test]
    fn test_consecutive_ltr() {
        let test_case = "foodpee";

        assert_eq!(check_consecutive_letters(test_case.to_string()), vec!['o','e']);
    }
}

fn check_consecutive_letters(letters: String) -> Vec<char> {
    let mut consecutive_letters: Vec<char> = Vec::new();
    let chars: Vec<char> = letters.chars().collect();

    for (idx, ch) in letters.chars().into_iter().enumerate() {
        if idx == 0 {
            continue;
        }
        
        if chars[idx-1] == ch{
            consecutive_letters.push(ch);
            let prev_ltr = ch;
        };
    };

    consecutive_letters
}

fn convert_word(word: String) -> String {
    let mut retval = String::new();

    for ch in word.chars() {
        let val = match ch {
            'u' => String::from("UwU"),
            'o' => String::from("OwO"),
            _ => ch.to_string(),
        };

        retval.push_str(val.as_str());
    }

    retval
}


fn main() {
    println!("Hello, world!");
}
