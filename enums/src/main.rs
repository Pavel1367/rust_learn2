mod enums2;
mod enums3;
mod shape;

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Green => TrafficLight::Red,
            TrafficLight::Red => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Green,
        }
    }
}
fn main() {
    println!("Hello, world!");
    println!("Next after red: {:?}", TrafficLight::Red.next())
}
