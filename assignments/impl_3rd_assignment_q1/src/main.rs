use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

struct PairTwo {
    x: u8,
    y: u8,
}

impl PairTwo {
    fn new(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl PairTwo {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
fn main() {
    
    let pair_01 = Pair::new(100,62);
    pair_01.cmp_display();

    let pair_02 = PairTwo::new(100,62);
    pair_02.cmp_display();

    /*
        In above example of two Types one is with Display and Partial Ord Bound with Generic T
        and other is with specific data type of u8.

        for the application of above comparison of values we need to bound it with Display and Partial Ord
        because we can have any type of data in Generic.

        While in second pair we used u8 type which is already Display and Partial Ord type.
        the drawback of this is we can use only U8 for the comparison of values x & y. but for 1st case
        we can easily compare any type of Numeric of stirng literal values.

    */

    
}
