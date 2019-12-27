use std::thread;
use std::time::Duration;
 
struct Cacher<T>
    where T: Fn(u32) -> (u32){
        calculation: T,
        value: Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32{

    fn new(calculation:T) -> Self{
        Self{
            calculation,
            value:None
        }
    }   
    
    fn getvalue(&mut self, x: u32) -> u32 {

        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation) (x);
                self.value = Some(v);
                v
            }
        }
        

    }
}

// fn generate_workout_plan(x: u32) -> u32 {

//     println!("Calculating workout....");
//     thread::sleep(Duration::from_secs(2));
//     x
// }

fn generate_workout(intensity: u32,random_num: u32){


    //let calculation = generate_workout_plan(intensity);

    // let generate_workout_plan = |x: u32| {

    //     println!("Calculating workout....");
    //     thread::sleep(Duration::from_secs(2));
    //     x
    // };
    
    // generate_workout_plan(intensity);

    // if intensity < 25 {
    //     // println!("Do {} Situps",generate_workout_plan(intensity));
    //     // println!("Do {} Pushups",generate_workout_plan(intensity));
    //     println!("Do {} Situps",intensity);
    //     println!("Do {} Pushups",intensity);
    // }
    // else {

    //     if random_num == 5 {
    //         println!("Take Rest!!");
    //     }
    //     else {
    //         //println!("Run {} Kms",generate_workout_plan(intensity))
    //         println!("Run {} Kms",intensity);
    //     }
    // }
}

fn main() {
    
    generate_workout(20,5);
}
