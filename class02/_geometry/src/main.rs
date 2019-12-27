#[derive(Debug)]
pub struct Circle{
    radius: f64
}

impl Circle {

    pub fn get_circle(radius: f64) -> Circle{

        Circle {radius}
    }

    pub fn circle_area(&self) -> f64 {

        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn circle_circumference(&self) -> f64 {

        2.0 * std::f64::consts::PI * self.radius
    }

    pub fn circle_diameter(&self) -> f64 {
        2.0 * self.radius
    }
}


fn main() {
    
    let circle_01 = Circle::get_circle(1.0);

    println!("Radius of Circle is {}",circle_01.circle_area());
    println!("Circumfarance of Circle is {}",circle_01.circle_circumference());
    println!("Diameter of Circle is {}",circle_01.circle_diameter());

}
