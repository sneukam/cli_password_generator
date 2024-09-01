// Generate a random password
mod generate_password;

use clap::Parser;
use generate_password::generate_passwords;

static MIN_PW_LENGTH: u8 = 20;
static MAX_PW_LENGTH: u8 = u8::MAX;
static MIN_NUM_PWS_GENERATED: u8 = 1;
static MAX_NUM_PWS_GENERATED: u8 = 30;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "45")]
    length: String,
    #[arg(short, long, default_value = "true")]
    extended_special_chars: String,
    #[arg(short, long, default_value = "10")]
    num_passwords_generated: String
}

fn main() {

    let args: Cli = Cli::parse();
    let length: u8 = get_pw_len(args.length);
    let num_pws: u8 = get_num_pws(args.num_passwords_generated);
    let ext_special: bool = get_ext_special(args.extended_special_chars);

    println!("\npw_length  \t\t{:?}\nextended_special_chars  {:?}\nnum_generated  \t\t{:?}\n", length, ext_special, num_pws);
    generate_passwords(length, num_pws, ext_special);
}

fn get_pw_len(pw_length: String) -> u8 {

    let length: u8 = pw_length.parse::<u8>().unwrap_or_else(|_| panic!("Error: password length must be an integer between {}-{}, inclusive.", MIN_PW_LENGTH, MAX_PW_LENGTH));

    if length < MIN_PW_LENGTH || length > MAX_PW_LENGTH {
        panic!("Error: password length must be an integer between {}-{}, inclusive.", MIN_PW_LENGTH, MAX_PW_LENGTH);
    }

    length
}

fn get_num_pws(num_passwords: String) -> u8 {

    let num_pws: u8 = num_passwords.parse::<u8>().unwrap_or_else(|_| panic!("Error: num passwords generated must be an integer between {}-{}, inclusive.", MIN_NUM_PWS_GENERATED, MAX_NUM_PWS_GENERATED));

    if num_pws < MIN_NUM_PWS_GENERATED || num_pws > MAX_NUM_PWS_GENERATED {
        panic!("Error: num passwords generated must be an integer between {}-{}, inclusive.", MIN_NUM_PWS_GENERATED, MAX_NUM_PWS_GENERATED);
    }

    num_pws
}

fn get_ext_special(ext_special_chars: String) -> bool {

    match ext_special_chars.to_lowercase().as_str() {
        "true" => return true,
        "false" => return false,
        _ => panic!("Error: The input value for extended_special_chars can only be 'true' or 'false'.")
    }
}




// for i in 0..length {
//     print!("{}", password[i as usize]);
// }
// println!("");