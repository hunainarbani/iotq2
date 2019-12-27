fn lifetime<'a>(x: &'a str, mut y: & 'a str) -> &'a str{

    {
        let x = "Dummy";
        y = &x;
    }

    y
}

fn main() {
    
    let aa = "Test";
    let bb = "Test 2";

    let rr = lifetime(&aa,&bb);

    println!("{}", rr );

}
