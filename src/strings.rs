// Two types of strings = [ Primitive str, String]

pub fn run() {
  let hello = "Hello"; // this is an example of Primative
  let greetings = String::from("Hello");

  println!("Length: {}", hello.len());
  println!("{}", hello);
  println!("Length: {}", greetings.len());
  println!("{}", greetings);
}