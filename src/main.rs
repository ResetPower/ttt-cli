mod calculate;

use calculate::calculate_winner;
use std::io;

fn clear_output() {
  for _ in 0..5 {
    print!("\x1bM");
  }
}

fn print_game(squares: [char; 9]) {
  let mut count = 0;
  for i in squares.iter() {
    if count == 3 {
      count = 0;
      println!()
    }
    print!("{} ", i);
    count += 1;
  }
  println!()
}

fn input(message: &str) -> usize {
  let mut input_text = String::new();
  print!("{}", message);
  io::Write::flush(&mut io::stdout()).unwrap();
  io::stdin().read_line(&mut input_text).unwrap();
  match input_text.trim().parse::<usize>() {
    Ok(i) => return i,
    Err(..) => return 0,
  };
}

fn main() {
  let mut squares: [char; 9] = ['n'; 9];
  println!("Welcome to the Rust Tic Tac Toe!");
  println!("Input 0 to exit.");
  println!(" === === === ");
  let mut next_is_x = true;
  loop {
    print_game(squares);
    let current = if next_is_x { 'x' } else { 'o' };
    let winner = calculate_winner(squares);
    if winner != 'n' {
      println!("Game Over! {} Won!", winner);
      break;
    } else {
      println!("Next player: {}", current)
    }
    let position = input("Next position (1-9): ");
    if position == 0 {
      break;
    }
    squares[position - 1] = current;
    next_is_x = !next_is_x;
    clear_output();
  }
}
