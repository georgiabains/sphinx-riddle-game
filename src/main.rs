use std::io;

fn main() {
  println!("The Sphinx asks you the following question:");

  let riddle = "There are two sisters: one gives birth to the other and she, in turn, gives birth to the first. Who are the two sisters?";
  let answer = "Day and night";

  println!("{riddle}");

  loop {
    println!("What is your answer?");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    println!("You guessed: {guess}");

    match guess {
      _ if guess.trim() == answer => {
        println!("you win");
        break;
      },
      _ => println!("Incorrect, try again")
    }
  }
}