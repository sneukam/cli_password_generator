use rand::Rng;

static ITERATION_RANGE: u32 = 4_000_000;

const STANDARD: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '!', '@', '#', '$', '%', '^', '&', '*'
];
const EXTENDED: &[char] = &[
    '~', '`', '(', ')', '-', '_', '+', '=', '{', '}', '[', ']', 
    '|', '\\', ':', ';', '"', '\'', ',', '<', '>', '.', '/', '?'
];

// return a vector of Strings that contain the passwords
pub fn generate_passwords(length: u8, num_pws: u8, ext_special: bool) -> u8 {
    
    // index range for allowed characters.
    let range = match ext_special {
        false => STANDARD.len(),
        true => STANDARD.len() + EXTENDED.len()
    };

    // Each int in the random_ints vector (contains ints in 0->range) will correlate with the characters in STANDARD and EXTENDED at the given index.
    let mut random_ints: Vec<u8> = Vec::<u8>::new();
    for _i in 0..ITERATION_RANGE as usize {
        let randomnum = rand::thread_rng().gen_range(0..range) as u8;
        random_ints.push(randomnum);
    }

    // create and store the passwords in Vec<Vec<char>>
    let mut passwords: Vec<Vec<char>> = Vec::new();
    for _i in 0..num_pws {
        let mut password: Vec<char> = Vec::<char>::new();
        for _j in 0..length {
            let rand_selection = rand::thread_rng().gen_range(0..ITERATION_RANGE);
            let char_num = random_ints[rand_selection as usize];
            let rand_char = get_char(char_num);
            password.push(rand_char);
        }
        passwords.push(password.clone());

        for i in 0..length {
            print!("{}", password[i as usize]);
        }
        println!("");
    }
    
    5
}

// return the char from the index in the STANDARD and/or EXTENDED char arrays.
fn get_char(index: u8) -> char {

    if index < STANDARD.len() as u8 {
        return STANDARD[index as usize];
    } else if index < STANDARD.len() as u8 + EXTENDED.len() as u8 {
        return EXTENDED[index as usize - STANDARD.len() as usize];
    }

    panic!("Error: Index out of range. Expected index to be between 0-{}, or {}-{}", STANDARD.len()-1, STANDARD.len(), EXTENDED.len());
}

