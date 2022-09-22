pub fn run() {
  // Print to console
  println!("Useful programmer in the print.rs file");

  // Basic formatting
  println!("{} is from {}", "Ian", "Nevada");

  // Positional Arguments
  println!("{0} is a {2} who likes learning {1}", "Ian", "code", "Programmer");

  // Named Arguments
  println!("{name} likes to {activity}", activity = "Surf", name = "Ian");

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 9, 8);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "uP"));

  // Basic Math
  println!("10 + 10 = {}", 10 + 10)
}