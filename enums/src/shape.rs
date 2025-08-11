pub enum Shape{
   Circle(u32),
    Rectangle(u32, u32),
    Triangle(u32, u32, u32),
}
pub const PI: f64 = std::f64::consts::PI;
impl Shape{
    pub fn square(&self) -> u32{
        match self {
            Shape::Circle(radius) =>( PI * radius.pow(2) as f64) as u32,
            Shape::Rectangle(a, b) => a * b,
            Shape::Triangle(a, b, c) => a*b*c
        }
    }
}