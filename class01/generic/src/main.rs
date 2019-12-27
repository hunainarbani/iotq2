fn main() {
    
    let numbers = vec![54,26,98,2,45,34];
    let chars = vec!['a','b','x'];

    // let mut largest = numbers[0];

    // for &item in numbers.iter() {
    //      if item > largest {
    //          largest = item;
    //      }
        
    // }

    
    println!("Largest Number {}",getlargestnumber(&numbers));
    println!("Largest Number {}",getlargestchar(&chars));
}

fn getlargestnumber(list: &[i32]) -> i32{

    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
       
   }

   largest


}

fn getlargestchar(list :&[char]) -> char{

    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
       
   }

   largest

}

// fn getlargest<T>(list :&[T]) -> T {

//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
       
//    }

//    largest
// }