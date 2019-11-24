#[derive(Debug)]
struct Spiderman{
    name: String,
}
#[derive(Debug)]
struct Superman{
    name: String,
}

#[derive(Debug)]
struct Batman{
    name: String,
}

#[derive(Debug)]
struct Hulk{
    name: String,
}

pub trait Power{
    fn power_score(&self)->i32;
}

impl Power for Superman {
    fn power_score(&self) ->i32 {
        100
    }
}


impl Power for Spiderman {
    fn power_score(&self) ->i32 {
        80
    }
}

impl Power for Batman {
    fn power_score(&self) ->i32 {
        50
    }
}

impl Power for Hulk {
    fn power_score(&self) ->i32 {
        50
    }
}

fn main() {
   
    let c1 = Superman{
        name: "Superman".to_string()
    };

    let c2= Spiderman{
        name: "Spiderman".to_string()
    };
    
    let c3= Batman{
        name: "Batman".to_string()
    };

    let c4 = Hulk{
        name: "Hulk".to_string()
    };

    println!("Power of {} is {}",c1.name, c1.power_score());
    println!("Power of {} is {}",c2.name, c2.power_score());
    println!("Power of {} is {}",c3.name, c3.power_score());
    println!("Power of {} is {}",c4.name, c4.power_score());

    
}
