
pub trait Calculation {
    fn Area(&self)-> f64;
    fn Diameter(&self)-> f64;
    fn Circumference(&self)-> f64;
}
#[derive(Debug)]
pub struct Circle{
    radius: f64
}


impl Circle {

    pub fn get_circle(radius: f64) -> Circle{

        Circle {radius}
    }
}

impl Calculation for Circle {

    fn Area(&self) -> f64 {

        std::f64::consts::PI * self.radius * self.radius
    }

    fn Circumference(&self) -> f64 {

        2.0 * std::f64::consts::PI * self.radius
    }

    fn Diameter(&self) -> f64 {
        2.0 * self.radius
    }
}


fn main() {
    
    let circle_01 = Circle::get_circle(1.0);

    println!("Radius of Circle is {}",circle_01.Area());
    println!("Circumfarance of Circle is {}",circle_01.Circumference());
    println!("Diameter of Circle is {}",circle_01.Diameter());

}
