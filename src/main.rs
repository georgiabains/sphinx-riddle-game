mod riddles;

use rand;
use std::fs::File;
use std::io::{stdin, BufReader, BufWriter, Write, Read};
use riddles::RIDDLES;
use data_encoding::BASE64;

fn main() { 

  // Return decoded question
  fn decode_question(index: usize) -> String {
    let encoded_riddle = BASE64.decode(RIDDLES[index].get_question()).unwrap();
    return String::from_utf8(encoded_riddle).expect("Found invalid UTF-8");
  }

  // Return array of decoded answers
  fn decode_answer(index: usize) -> Vec<String> {
    let encoded_answer_array = RIDDLES[index].get_answer();
    let mut decoded_answer_array: Vec<String> = Vec::with_capacity(encoded_answer_array.len());

    for encoded_answer in encoded_answer_array.iter() {
      let answer = BASE64.decode(encoded_answer).unwrap();
      let decoded_answer = String::from_utf8(answer).expect("Found invalid UTF-8");
      decoded_answer_array.push(decoded_answer);
    }

    return decoded_answer_array;
  }

  let data = "Some data hello world!";
  let f = File::create("./data/save.custom").expect("Should be able to create file");
  let mut f = BufWriter::new(f);
  f.write_all(data.as_bytes()).expect("Should be able to write data");

  f.flush().unwrap();

  let mut save_data = String::new();
  let save = File::open("./data/save.custom").expect("Should be able to open `./data/save.custom`");
  let mut br = BufReader::new(save);
  br.read_to_string(&mut save_data).expect("Should be able to read to string");
  println!("{}", save_data);

  let riddle_index: usize = rand::random_range(0..RIDDLES.len());
  let riddle = decode_question(riddle_index);
  let answer = decode_answer(riddle_index);

  // Sphinx
  println!("The Sphinx asks you the following question:");
  println!("{riddle}");

  // Main gameplay loop
  loop {
    println!("What is your answer?");

    let mut guess = String::new();

    stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    // Sanitise guess (remove whitespace, convert to lowercase)
    let mut sanitised_guess = String::from(guess.trim());
    sanitised_guess.retain(|c| c != ' ');
    sanitised_guess = sanitised_guess.to_lowercase();

    println!("You guessed: {guess}");

    match sanitised_guess {
      _ if answer.contains(&sanitised_guess) => {
        println!("you win");
        break;
      },
      _ => println!("Incorrect, try again")
    }
  }
}