//use std::fmt::Display;
//use std::cmp::Ordering;

//#[derive(Debug)]
// struct Values<T>{
//     n: T
// }

// impl<T> Values<T>{

//     fn new(n: T)-> Self{
//         Self {n
//         }
//     }
// }

// impl<T: Display + PartialOrd> Values<T>{
    
//     fn get_average(self) ->{

//     }
// }

fn get_average(listv: &[i32]) -> i32{


}


fn main() {
    
    let values_01 = [3,7,5,13,20,23,39,23,40,23,14,12,56,23,29];
    let count = values_01.len();
    let mut sum = 0;

    for i in 0..count{
        sum += values_01[i];
    }

    println!("The average is {}", sum/count);


    

}

