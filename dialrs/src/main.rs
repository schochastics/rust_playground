use phonenumber::Mode;
use std::env;

// fn main() {
//     let mut args = env::args().skip(1).collect::<Vec<_>>();

//     if args.len() < 1 {
//         panic!("not enough arguments");
//     }
//     println!("{}", args[0]);
//     let number = args.pop().unwrap();
//     let country = args.pop().map(|c| c.parse().unwrap());
//     println!("number: {}", number);
//     println!("country: {:?}", country);
//     let number = phonenumber::parse(country, number).unwrap();
//     let valid = phonenumber::is_valid(&number);

//     if valid {
//         println!("\x1b[32m{:#?}\x1b[0m", number);
//         println!();
//         println!(
//             "International: {}",
//             number.format().mode(Mode::International)
//         );
//         println!("     National: {}", number.format().mode(Mode::National));
//         println!("      RFC3966: {}", number.format().mode(Mode::Rfc3966));
//         println!("        E.164: {}", number.format().mode(Mode::E164));
//     } else {
//         println!("\x1b[31m{:#?}\x1b[0m", number);
//     }
// }

fn main() {
    // Directly using the phone number as input
    let input_number = "+491765545541";

    // Assuming the country code might not always be provided, we handle it as an Option
    let country = None; // Example: for Germany, you might use Some(49)

    let number = phonenumber::parse(country, input_number).unwrap();
    let valid = phonenumber::is_valid(&number);
    println!("{}", valid);
    if valid {
        println!("\x1b[32m{:#?}\x1b[0m", number);
        println!();
        println!(
            "International: {}",
            number.format().mode(phonenumber::Mode::International)
        );
        println!(
            "     National: {}",
            number.format().mode(phonenumber::Mode::National)
        );
        println!(
            "      RFC3966: {}",
            number.format().mode(phonenumber::Mode::Rfc3966)
        );
        println!(
            "        E.164: {}",
            number.format().mode(phonenumber::Mode::E164)
        );
    } else {
        println!("\x1b[31m{:#?}\x1b[0m", number);
    }
}
