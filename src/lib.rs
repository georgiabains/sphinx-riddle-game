pub struct Riddle<'a> {
  question: &'a str,
  answer: &'a str,
}

impl Riddle<'_> {

  // Return a Riddle's question
  pub fn get_question(&self) -> &str {
    self.question
  }

  // Return a Riddle's answer
  pub fn get_answer(&self) -> &str {
    self.answer
  }

  // Create new Riddle
  pub const fn new<'a>(q: &'a str, a: &'a str) -> Riddle<'a> {
    Riddle { question: &q, answer: &a }
  }
}