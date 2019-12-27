// #[derive(Debug)]
//  struct Circle{
//     radius: f64
// }
// impl Circle{
//     fn new(r: f64) -> Circle {
//         Circle {
//             radius: r
//         }
//     }
// }
// #[derive(Debug)]
//  struct Square{
//     side: f64
// }
// impl Square {
//     fn new(s: f64) -> Square{
//         Square {
//             side: s
//         }
//     }
// }
// trait Area {
//     fn get_area(&self) -> f64 ;    
// }
// trait Diameter{    
//     fn get_diameter(&self)-> f64;    
// }
// trait Perimeter{
//     fn get_perimeter(&self)->f64;    
// }
// trait Circumference{
//     fn get_circumference(&self)->f64;
// }

// fn get_square_info<T>(item: T) -> String
//     where T: Area + Perimeter
// {
//     format!("Area of Square is {}\nPerimter of Square is {}"
//     ,item.get_area()
//     ,item.get_perimeter())
// }


// fn get_circle_info<T>(item: T) -> String
//     where T: Area + Diameter + Circumference
// {
//     format!("Area of Circle is {}\nDiameter of Circle is {}\nCircumference of Circle is {}\n"
//     ,item.get_area()
//     ,item.get_diameter()
//     ,item.get_circumference())
// }



// impl Area for Circle{
//     fn get_area(&self) -> f64 {
//         std::f64::consts::PI * self.radius * self.radius
//     }
// }

// impl Diameter for Circle{
//     fn get_diameter(&self) -> f64{
//         2.0 * self.radius
//     }
// }

// impl Circumference for Circle { 
//     fn get_circumference(&self) -> f64{
//         2.0 * std::f64::consts::PI * self.radius
//     }
// }

// impl Perimeter for Square {
//     fn get_perimeter(&self) -> f64{
//         4.0 * self.side
//     }
// }

// impl Area for Square{
//     fn get_area(&self) -> f64 {
//         self.side * self.side
//     }
// }

// fn main() {
    
//     let circle_01 = Circle::new(2.0);
//     //println!("Area of Circle is {}",circle_01.get_area());

//     let square_01 = Square::new(4.0);
//     //println!("Area of Square is {}",square_01.get_area())

//     println!("{}",get_circle_info(circle_01));
//     println!("{}",get_square_info(square_01));

// }


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
#[derive(Debug)]
 struct Square<T>{
    side: T
}
impl<T> Square<T> {
    fn new(s: T) -> Self{
        Self {
            side: s
        }
    }
}
trait Area {
    fn get_area<T>(&self) -> U ;    
}
trait Diameter{    
    fn get_diameter<T>(&self)-> U;    
}
trait Perimeter{
    fn get_perimeter<T>(&self)->U;    
}
trait Circumference{
    fn get_circumference<T>(&self)->U;
}

fn get_square_info<T>(item: T) -> String
    where T: Area + Perimeter
{
    format!("Area of Square is {}\nPerimter of Square is {}"
    ,item.get_area()
    ,item.get_perimeter())
}


fn get_circle_info<T>(item: T) -> String
    where T: Area + Diameter + Circumference
{
    format!("Area of Circle is {}\nDiameter of Circle is {}\nCircumference of Circle is {}\n"
    ,item.get_area()
    ,item.get_diameter()
    ,item.get_circumference())
}



impl<U,T> Area for Circle<T>{
    fn get_area(&self) -> U {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Diameter for Circle<T>{
    fn get_diameter(&self) -> U{
        2.0 * self.radius
    }
}

impl Circumference for Circle<T> { 
    fn get_circumference(&self) -> U{
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Perimeter for Square<T> {
    fn get_perimeter(&self) -> U{
        4.0 * self.side
    }
}

impl Area for Square<T>{
    fn get_area(&self) -> U {
        self.side * self.side
    }
}

fn main() {
    
    let circle_01 = Circle::new(2.0);
    //println!("Area of Circle is {}",circle_01.get_area());

    let square_01 = Square::new(4.0);
    //println!("Area of Square is {}",square_01.get_area())

    println!("{}",get_circle_info(circle_01));
    println!("{}",get_square_info(square_01));

}
