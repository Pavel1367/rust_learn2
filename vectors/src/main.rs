use std::collections::VecDeque;

mod practice;
mod task_1;
mod task_2;
mod task_3;
mod task_4;
mod task_5;


fn main() {
    println!("Hello, world!");
    let mut average = MovingAverage::new(3);
    println!("{}", average.next(2.0));
    println!("{}", average.next(5.0));
    println!("{}", average.next(6.9));
    println!("{}", average.next(10.0));
    println!("{:?}", average);

}


#[derive(Debug)]
struct MovingAverage{
    window: VecDeque<f64>,
    capacity: usize
}

impl MovingAverage{
    fn new(capacity: usize)->MovingAverage{
        MovingAverage{
            capacity,
            window: VecDeque::new()
        }
    }

    fn next(&mut self, number: f64)-> f64{
        if self.window.len() == self.capacity{
            self.window.pop_front();
        }
        self.window.push_back(number);
        let mut sum:f64 = 0.0;
        for element in self.window.iter(){
            sum += element
        }
        sum / (self.window.len()) as f64
    }
}