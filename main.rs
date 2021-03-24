use std::io;

const shipname: &str = "Reefship";
const shiptype: &str = "Small Sloop";

fn main() {
  let mut shiptype = "Fishing Boat";
  let mut hp = 50;
  let mut atk = 50;
  let mut cannon1 = "";
  let mut cannon2 = "";

  start();
  loop {
    game();
  }
}

fn game() {
  input(shipname);
}

fn start() {
  println!("Welcome to Reef!");
  println!("In this game, you play as a pirate ship travelling through the sea.");
  input("First, what would you like to name your ship?\n<=======<x>=======>\n");
  print!("{}[2J", 27 as char);
  println!("You set off into the ocean with a fishing boat, equipped with a small cannon and carrying 50 gold coins.")
}

fn input(prompt: &str) -> String {
  println!("{}", prompt);
  let mut inputcontent = String::new();
  io::stdin().read_line(&mut inputcontent)
    .expect("Failed to accept input...");
  return inputcontent;
}