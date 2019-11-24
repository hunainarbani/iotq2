
#[derive(Debug)]
struct Points<X,Y> {
    x: X,
    y: Y
}



fn main() {
    
    let p1 = Points{x: 10, y:11.5};

    println!("{:?}",p1);

}
