mod riddles;

use std::io;
use riddles::RIDDLES;
use data_encoding::BASE64;

fn main() { 
  let riddle_index = 0; // TODO: Choose random index

  // Decode riddle's question or answer
  fn decode_riddle(index: usize, is_question: bool) -> String {
    let encoded_riddle = RIDDLES[index];
    let value;

    if is_question {
      value = BASE64.decode(encoded_riddle.get_question()).unwrap();
    } else {
      value = BASE64.decode(encoded_riddle.get_answer()).unwrap();
    }

    return String::from_utf8(value).expect("Found invalid UTF-8");
  }

  let riddle = decode_riddle(riddle_index, true);
  let answer = decode_riddle(riddle_index, false);

  // Sphinx
  println!("The Sphinx asks you the following question:");
  println!("{riddle}");

  // Main gameplay loop
  loop {
    println!("What is your answer?");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    // Sanitise guess (remove whitespace, convert to lowercase)
    let mut sanitised_guess = String::from(guess.trim());
    sanitised_guess.retain(|c| c != ' ');
    sanitised_guess = sanitised_guess.to_lowercase();

    println!("You guessed: {guess}");

    match sanitised_guess {
      _ if answer.contains(&sanitised_guess.as_str()) => {
        println!("you win");
        break;
      },
      _ => println!("Incorrect, try again")
    }
  }
}