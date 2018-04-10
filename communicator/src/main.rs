extern crate communicator;

use communicator::client::connect;

#[derive(Debug)]
enum MagicEightBall {
  Yes,
  No,
  Maybe,
}

use MagicEightBall::*;

fn main() {
  connect();
  println!("{:?}", Yes);
  println!("{:?}", No);
  println!("{:?}", Maybe);
}
