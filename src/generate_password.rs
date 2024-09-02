use rand::Rng;

static RANDOMIZED_VECTOR_SIZE: usize = 10_000_000;

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
pub fn generate_passwords(length: u8, num_pws: u8, ext_special: bool) -> Vec<Vec<char>> {
    
    // index range for allowed characters (-> using just STANDARD characters or EXTENDED special characters as well)
    let range = match ext_special {
        false => STANDARD.len(),
        true => STANDARD.len() + EXTENDED.len()
    };

    // Fill a Vector (random_ints) with random integers in the range specified above.
    // This will be used later to select random characters from.
    let mut random_ints: Vec<u8> = Vec::<u8>::new();
    for _i in 0..RANDOMIZED_VECTOR_SIZE as usize {
        let randomnum = rand::thread_rng().gen_range(0..range) as u8;
        random_ints.push(randomnum);
    }

    // create and store the passwords in Vec<Vec<char>>
    // randomization: Each char in the password is selected by picking a random index in random_ints, 
    // then converting the int value at that location to its associated char value from STANDARD and/or EXTENDED char arrays.
    let mut passwords: Vec<Vec<char>> = Vec::new();
    for _i in 0..num_pws {
        let mut password: Vec<char> = Vec::<char>::new();
        for _j in 0..length {
            let rand_selection = rand::thread_rng().gen_range(0..RANDOMIZED_VECTOR_SIZE);
            let char_num = random_ints[rand_selection];
            let rand_char = get_char(char_num as usize);
            password.push(rand_char);
        }
        passwords.push(password);
    }
    
    passwords
}

// return the char from the index in the STANDARD or (if applicable) EXTENDED char arrays.
fn get_char(index: usize) -> char {

    if index < STANDARD.len() {
        return STANDARD[index];
    } else if index < STANDARD.len() + EXTENDED.len() {
        return EXTENDED[index - STANDARD.len()];
    }

    panic!("Error: Index out of range. Expected index to be between 0-{}, or {}-{}", STANDARD.len()-1, STANDARD.len(), EXTENDED.len());
}
