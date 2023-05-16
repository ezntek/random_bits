fn encode_letter(character: &char) -> Result<&'static str, &str> {
    match character {
        ' ' => Ok("_"),
        'a' => Ok("me,meow"),
        'b' => Ok("meow,me,me,meow"),
        'c' => Ok("meow,me,meow,me"),
        'd' => Ok("meow,me,me"),
        'e' => Ok("me"),
        'f' => Ok("me,me,meow,me"),
        'g' => Ok("meow,meow,me"),
        'h' => Ok("me,me,me,me"),
        'i' => Ok("me,me"),
        'j' => Ok("me,meow,meow,meow"),
        'k' => Ok("meow,me,meow"),
        'l' => Ok("me,meow,me,me"),
        'm' => Ok("meow,meow"),
        'n' => Ok("meow,me"),
        'o' => Ok("meow,meow,meow"),
        'p' => Ok("me,meow,meow,me"),
        'q' => Ok("meow,meow,me,meow"),
        'r' => Ok("me,meow,me"),
        's' => Ok("me,me,me"),
        't' => Ok("meow"),
        'u' => Ok("me,me,meow"),
        'v' => Ok("me,me,me,meow"),
        'w' => Ok("me,meow,meow"),
        'x' => Ok("meow,me,me,meow"),
        'y' => Ok("meow,me,meow,meow"),
        'z' => Ok("meow,meow,me,me"),
        '1' => Ok("me,meow,meow,meow,meow"),
        '2' => Ok("me,me,meow,meow,meow"),
        '3' => Ok("me,me,me,meow,meow"),
        '4' => Ok("me,me,me,me,meow"),
        '5' => Ok("me,me,me,me,me"),
        '6' => Ok("meow,me,me,me,me"),
        '7' => Ok("meow,meow,me,me,me"),
        '8' => Ok("meow,meow,meow,me,me"),
        '9' => Ok("meow,meow,meow,meow,me"),
        '0' => Ok("meow,meow,meow,meow,me"),
        _ => Err("Invalid character encountered!"),
    }
}

fn decode_sequence(sequence: &String) -> Result<char, &'static str> {
    match sequence.as_str() {
        "_" => Ok(' '),
        "me,meow" => Ok('a'),
        "meow,me,me,meow" => Ok('b'),
        "meow,me,meow,me" => Ok('c'),
        "meow,me,me" => Ok('d'),
        "me" => Ok('e'),
        "me,me,meow,me" => Ok('f'),
        "meow,meow,me" => Ok('g'),
        "me,me,me,me" => Ok('h'),
        "me,me" => Ok('i'),
        "me,meow,meow,meow" => Ok('j'),
        "meow,me,meow" => Ok('k'),
        "me,meow,me,me" => Ok('l'),
        "meow,meow" => Ok('m'),
        "meow,me" => Ok('n'),
        "meow,meow,meow" => Ok('o'),
        "me,meow,meow,me" => Ok('p'),
        "meow,meow,me,meow" => Ok('q'),
        "me,meow,me" => Ok('r'),
        "me,me,me" => Ok('s'),
        "meow" => Ok('t'),
        "me,me,meow" => Ok('u'),
        "me,me,me,meow" => Ok('v'),
        "me,meow,meow" => Ok('w'),
        #[allow(unreachable_patterns)] // why? don't know, rust-analyzer's shenanigans.
        "meow,me,me,meow" => Ok('x'),
        "meow,me,meow,meow" => Ok('y'),
        "meow,meow,me,me" => Ok('z'),
        "me,meow,meow,meow,meow" => Ok('1'),
        "me,me,meow,meow,meow" => Ok('2'),
        "me,me,me,meow,meow" => Ok('3'),
        "me,me,me,me,meow" => Ok('4'),
        "me,me,me,me,me" => Ok('5'),
        "meow,me,me,me,me" => Ok('6'),
        "meow,meow,me,me,me" => Ok('7'),
        "meow,meow,meow,me,me" => Ok('8'),
        "meow,meow,meow,meow,me" => Ok('9'),
        #[allow(unreachable_patterns)] // why? don't know, r-a's shenanigans.
        "meow,meow,meow,meow,me" => Ok('0'),
        _ => Err("Invalid code encountered!"),
    }
}

pub fn encode(text: &String) -> String {
    let mut result = String::new();

    for ch in text.chars().into_iter() {
        let translated_ch = match encode_letter(&ch.to_ascii_lowercase()) {
            Ok(c) => format!("{} ; ", c),
            Err(_) => String::new(),
        };

        result.push_str(translated_ch.as_str())
    };

    result
}

pub fn decode(text: &String) -> String {
    let mut result = String::new();
    let sliced_text: Vec<String> = text.split(" ; ")
        .map(|text| text.to_ascii_lowercase())
        .collect();

    for pat in sliced_text.into_iter() {
        let ch = match decode_sequence(&pat) {
            Ok(pt) => pt,
            Err(_) => ' ',
        };

        result.push(ch);
    };
    
    result
}