use std::io;

fn main() {
    
    println!(r#"  __  __                     _        _____ _       _               
    |  \/  |                   ( )      / ____(_)     | |              
    | \  / | ___  _ __ ___  ___|/ ___  | |     _ _ __ | |__   ___ _ __ 
    | |\/| |/ _ \| '__/ __|/ _ \ / __| | |    | | '_ \| '_ \ / _ \ '__|
    | |  | | (_) | |  \__ \  __/ \__ \ | |____| | |_) | | | |  __/ |   
    |_|  |_|\___/|_|  |___/\___| |___/  \_____|_| .__/|_| |_|\___|_|   
                                                | |                    
                                                |_|                   "#);

    let mut option: i32 = 0;

    while option != -1 {
        println!("Choose from below options:");
        println!("1 - Translate English To Morse Code");
        println!("2 - Translate Morse Code TO English");
        println!("Any number other than 1 or 2 to exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        option = input.trim().parse().expect("Please enter a number");

        match option {
            1 => {
                println!("Enter a string to translate to Morse code:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let morse_code = translate_to_morse(&input.trim());
                println!("Morse code: {}", morse_code);
            }
            2 => {
                println!("Enter Morse code to translate to English:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let english = morse_to_english(&input.trim());
                println!("English: {}", english);
            }
            _ => {
                println!("Exiting...");
                break; // Exit the loop when any other number is pressed
            }
        }
    }
}

fn translate_to_morse(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        result.push_str(match c.to_ascii_uppercase() {
            'A' => ".-", 'B' => "-...", 'C' => "-.-.", 'D' => "-..", 'E' => ".", 'F' => "..-.", 'G' => "--.", 'H' => "....",
            'I' => "..", 'J' => ".---", 'K' => "-.-", 'L' => ".-..", 'M' => "--", 'N' => "-.", 'O' => "---", 'P' => ".--.",
            'Q' => "--.-", 'R' => ".-.", 'S' => "...", 'T' => "-", 'U' => "..-", 'V' => "...-", 'W' => ".--", 'X' => "-..-",
            'Y' => "-.--", 'Z' => "--..",
            '0' => "-----", '1' => ".----", '2' => "..---", '3' => "...--", '4' => "....-", '5' => ".....",
            '6' => "-....", '7' => "--...", '8' => "---..", '9' => "----.",
            ' ' => "/", // Using / as a separator between words
            _ => "" // Ignore characters that are not in Morse code
        });
        result.push(' '); // Adding space between Morse code characters
    }
    result.trim().to_string()
}

fn morse_to_english(input: &str) -> String {
    let mut result = String::new();
    let words: Vec<&str> = input.split('/').collect();
    for word in words {
        let chars: Vec<&str> = word.split(' ').collect();
        for c in chars {
            result.push_str(match c {
                ".-" => "A", "-..." => "B", "-.-." => "C", "-.." => "D", "." => "E",
                "..-." => "F", "--." => "G", "...." => "H", ".." => "I", ".---" => "J",
                "-.-" => "K", ".-.." => "L", "--" => "M", "-." => "N", "---" => "O",
                ".--." => "P", "--.-" => "Q", ".-." => "R", "..." => "S", "-" => "T",
                "..-" => "U", "...-" => "V", ".--" => "W", "-..-" => "X", "-.--" => "Y",
                "--.." => "Z", _ => "" // Ignore characters that are not in Morse code
            });
        }
        result.push(' '); // Adding space between words
    }
    result.trim().to_string()
}
 
