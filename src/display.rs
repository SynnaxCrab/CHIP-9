const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.screen[x + y * WIDTH] = on as u8;
    }

    fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.screen[x + y * WIDTH] == 1
    }

    pub fn cls(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, false);
            }
        }
    }
}
