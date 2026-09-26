mod riddles;

use std::io;
use riddles::RIDDLES;

use data_encoding::BASE64;

fn main() {
  println!("The Sphinx asks you the following question:");

  let riddle = "There are two sisters: one gives birth to the other and she, in turn, gives birth to the first. Who are the two sisters?";
  let answer = ["dayandnight", "day&night"];

  // println!("{}", RIDDLES[0].get_question().to_string());
  // println!("{}", RIDDLES[0].get_answer().to_string());

  let test = BASE64.decode(RIDDLES[0].get_answer()).unwrap();

  let test2 = String::from_utf8(test).expect("Found invalid UTF-8");

  println!("{}", test2);

  // println!("{:?}", BASE64.decode(RIDDLES[0].get_question()).unwrap());
  // println!("{:?}", BASE64.decode(RIDDLES[0].get_answer()).unwrap());

  // assert_eq!(BASE64.decode(RIDDLES[0].get_question()).unwrap(), b"hello");

  println!("{riddle}");

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