fn decimal_to_binary(decimal: u32) -> String {
    let mut binary = String::new();
    let mut n = decimal;

    while n > 0 {
        binary.push_str(&(n % 2).to_string());
        n /= 2;
    }

    binary.chars().rev().collect::<String>()
}

fn binary_to_decimal(binary: &str) -> u32 {
    let reversed_binary = binary.chars().rev().collect::<String>();
    let mut decimal = 0;

    for (i, c) in reversed_binary.chars().enumerate() {
        if c == '1' {
            decimal += 2u32.pow(i as u32);
        }
    }

    decimal
}

fn main() {
    println!("Binary Converter");

    loop {
        println!("Select an option:");
        println!("1. Convert decimal to binary");
        println!("2. Convert binary to decimal");
        println!("3. Exit");

        let mut choice = String::new();

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim().parse::<u32>();

        match choice {
            Ok(1) => {
                println!("Enter a decimal number:");
                let mut decimal_input = String::new();
                std::io::stdin()
                    .read_line(&mut decimal_input)
                    .expect("Failed to read line");

                let decimal_number = decimal_input.trim().parse::<u32>();

                match decimal_number {
                    Ok(number) => {
                        let converted_binary = decimal_to_binary(number);
                        println!("Decimal to Binary: {}", converted_binary);
                    }
                    Err(_) => println!("Invalid decimal number input"),
                }
            }
            Ok(2) => {
                println!("Enter a binary number:");
                let mut binary_input = String::new();
                std::io::stdin()
                    .read_line(&mut binary_input)
                    .expect("Failed to read line");

                let binary_number = binary_input.trim();

                let converted_decimal = binary_to_decimal(binary_number);
                println!("Binary to Decimal: {}", converted_decimal);
            }
            Ok(3) => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

