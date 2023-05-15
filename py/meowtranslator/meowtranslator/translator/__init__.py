LOOKUP = {
    " ": "_",
    "a": "me,meow",
    "b": "meow,me,me,meow",
    "c": "meow,me,meow,me",
    "d": "meow,me,me",
    "e": "me",
    "f": "me,me,meow,me",
    "g": "meow,meow,me",
    "h": "me,me,me,me",
    "i": "me,me",
    "j": "me,meow,meow,meow",
    "k": "meow,me,meow",
    "l": "me,meow,me,me",
    "m": "meow,meow",
    "n": "meow,me",
    "o": "meow,meow,meow",
    "p": "me,meow,meow,me",
    "q": "meow,meow,me,meow",
    "r": "me,meow,me",
    "s": "me,me,me",
    "t": "meow",
    "u": "me,me,meow",
    "v": "me,me,me,meow",
    "w": "me,meow,meow",
    "x": "meow,me,me,meow",
    "y": "meow,me,meow,meow",
    "z": "meow,meow,me,me",
    "1": "me,meow,meow,meow,meow",
    "2": "me,me,meow,meow,meow",
    "3": "me,me,me,meow,meow",
    "4": "me,me,me,me,meow",
    "5": "me,me,me,me,me",
    "6": "meow,me,me,me,me",
    "7": "meow,meow,me,me,me",
    "8": "meow,meow,meow,me,me",
    "9": "meow,meow,meow,meow,me",
    "0": "meow,meow,meow,meow,me",
}

def translate(input: str):
    retval: str = ""
    for char in input:
        try:
            retval += f"{LOOKUP[char].upper()} ; "
        except KeyError:
            retval += char
    return retval

def get_key_from_value(value: str) -> str:
    for (key, val) in LOOKUP.items():
        if val == value:
            return key
    return ""

def decode(input: str) -> str:
    split_input = input.split(" ; ")
    retval: str = ""

    for sequence in split_input:
        retval += get_key_from_value(sequence.lower())
    
    return retval
    