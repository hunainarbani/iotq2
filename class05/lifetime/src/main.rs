fn main() {
    
    println!("{}",add_to_numbers(&mut 55,&40));


}


fn add_to_numbers<'a,'b>(x: &'a mut i32,y: &'b i32) -> &'a i32{

    *x = *x + y;
    x

}