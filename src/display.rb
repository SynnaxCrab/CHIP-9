const WIDTH: usize = 64;
const HEIGHT: usize = 32;

struct Display {
  screen: [u8; WIDTH * HEIGHT]
}

impl Display {
  fn new() -> Display {
    Display {
      screen: [0; WIDTH * HEIGHT]
    }
  }
}
