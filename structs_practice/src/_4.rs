struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Дополните конструктор
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // Дополните метод для вычисления расстояния до другой точки
    fn distance_to(&self, another_point: &Point) -> f64 {
        let diff_x = self.x - another_point.x;
        let diff_y = self.y - another_point.y;
        (diff_x.pow(2) + diff_y.pow(2)) as f64
    }
}
