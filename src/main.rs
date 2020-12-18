mod begginer;

use crate::begginer::{
  bin2dec::bin2dec
};
use std::env;


fn main() {
  let args: Vec<String> = env::args().collect();

  let user_input = String::from(&args[1]);

  bin2dec(user_input);
}
