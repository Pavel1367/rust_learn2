struct Color(u8, u8, u8);

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    fn red(&self) -> u8 {
        *self.0
    }

    fn green(&self) -> u8 {
        *self.1
    }

    fn blue(&self) -> u8 {
        *self.2
    }
}
