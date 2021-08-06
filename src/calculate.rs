pub fn calculate_winner(squares: [char; 9]) -> char {
  let lines = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
  ];
  for i in lines.iter() {
    let a = i[0];
    let b = i[1];
    let c = i[2];
    if squares[a] != 'n' && squares[a] == squares[b] && squares[a] == squares[c] {
      return squares[a];
    }
  }
  'n'
}
