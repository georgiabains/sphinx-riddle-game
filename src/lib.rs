use serde::{Deserialize, Serialize};
use serde_json::Result;

pub struct Riddle<'a> {
  question: &'a [u8],
  answer: &'a[&'a [u8]],
}

impl Riddle<'_> {

  // Return a Riddle's question
  pub fn get_question(&self) -> &[u8] {
    self.question
  }

  // Return a Riddle's answer
  pub fn get_answer(&self) -> &[&[u8]] {
    self.answer
  }

  // Create new Riddle
  pub const fn new<'a>(q: &'a [u8], a: &'a [&'a [u8]]) -> Riddle<'a> {
    Riddle { question: &q, answer: &a }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveData {
  answered_riddle_indices: Vec<usize>,
}

impl SaveData {
  pub fn get_answered_riddle_indices(&self) -> Vec<usize> {
    self.answered_riddle_indices.clone()
  }

  pub const fn new() -> SaveData {
    SaveData { answered_riddle_indices: vec![] }
  }

  pub fn add_riddle_index(&mut self, index: usize) -> () {
    self.answered_riddle_indices.push(index)
  }
}