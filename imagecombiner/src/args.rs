pub fn nth_arg(n: usize) -> String {
  return std::env::args().nth(n).unwrap_or_else(|| "".to_string());
}

#[derive(Debug)]
pub struct Args {
  pub img1: String,
  pub img2: String,
  pub out: String
}

impl Args {
  pub fn new() -> Self {
    Args {
      img1: nth_arg(1),
      img2: nth_arg(2),
      out: nth_arg(3)
    }
  }
}