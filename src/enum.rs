fn main () {
    let rect = Shape::Rectangle(2.2, 1.2);
    calculate_area(rect);
    let circle = Shape::Circle(10.0);
    calculate_area(circle);
}

enum Shape {
    Rectangle(f64,f64),
    Circle(f64)
}


fn calculate_area(sh:Shape)->f64{
    let area = match  sh {
        Shape::Rectangle(a,b) => a * b,
        Shape::Circle(a) => a*a*3.14,
    };
    println!("{}",area);
    return area;
}