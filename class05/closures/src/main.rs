fn main() {

    let pi = 3.1415;

    let area_circlec = |r :f64|{
        
        pi * (r * r)
    };

    fn area_circlef(r: f64) -> f64{

        3.1415 * (r * r)
    }

    println!("Area by Closure {}",area_circlec(4.3));
    println!("Area by Function {}",area_circlef(4.3));
}


