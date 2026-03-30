#[derive(Debug, Clone, Copy)]
pub enum Color {
  White,
  Black,
}
impl Color {
  pub fn next(&self) -> Self {
    match self {
      Color::White => Color::Black,
      Color::Black => Color::White,
    }
  }
}
impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    matches!((self, other), (Color::White, Color::White) | (Color::Black, Color::Black))
  }
}
