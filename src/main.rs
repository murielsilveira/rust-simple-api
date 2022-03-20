#![allow(dead_code)]

use std::fmt;

mod learn;

fn main() {
  // learn::guessing_game::start();
  // learn::fibonacci::print_first_ten();
  // learn::temperature::print_conversions();
  learn::slices::print_randon_string_slices_info();
  // printing_structs();
}

fn printing_structs() {
  let r1 = Rectangle {
    width: 50,
    height: 40,
  };
  let r2 = Rectangle::square(50);

  println!("Debug print: {:?}", r1);
  println!("Display print: {}", r1);
  println!("Pretty debug print: {:#?}", r1);

  dbg!(&r1);
  let _ = dbg!(r1.area());

  println!("{} can hold {}: {}", r1, r1, r1.can_hold(&r1));
  println!("{} can hold {}: {}", r1, r2, r1.can_hold(&r2));
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl fmt::Display for Rectangle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Rect({}, {})", self.width, self.height)
  }
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, r: &Rectangle) -> bool {
    self.width >= r.width && self.height >= r.height
  }
}
