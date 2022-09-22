pub fn run() {
  /*
  Multiple line comment
  */

  // Default to "i32" so you don't have to specify it...
  let x = 1;

  // Default to "f64"
  let y = 2.5;

  let z: i64 = 454545445454;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  // Get boolean from expression
  let is_greater: bool = 10 < 5;

  // Char are 
  let a1 = 'a';

  // Works with emojis
  let face = '\u{1F600}';

  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}