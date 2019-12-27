fn main() {
    let list_int = vec![23,56,90];
    let list_char = vec!['a','e','z','i','o','u'];

    println!("Largest value in Integer list is {}", getlargestvalue(&list_int));
    println!("Largest value in Character list is {}", getlargestvalue(&list_char));
}


//fn getlargestvalue<T> (item: &[T]) -> T {)
    fn getlargestvalue<T: PartialOrd + Copy> (item: &[T]) -> T {
    let mut largest = item[0];

    for &value in item.iter(){

        if value > largest {
            largest = value;
        }
    }

    largest
}