#[derive(Debug)]
 struct Circle<T>{
    radius: T
}
impl<T> Circle<T>{
    fn new(r: T) -> Self {
        Self {
            radius: r
        }
    }
}

trait Area {
    type Output;
    fn get_area(&self, _:U) -> Self::Output ;    
}

impl Area for Circle<U>{
    fn get_area(&self) -> Self::Output {
        self.radius * self.radius
    }
}

// impl Area for Circle<T>{
//     fn get_area(&self) -> T {
//         std::f64::consts::PI * self.radius * self.radius
//     }
// }

fn main() {
    
    let circle_01 = Circle::new(3);

    //println!("{}",circle_01);
}
