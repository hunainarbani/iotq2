// #[derive(Debug)]
// enum typeoflicense{
//     HTV,
//     LTV
// }
#[derive(Debug)]
pub struct Driver{
    name: String,
    experience: u8,    
    ltype: String//typeoflicense
}

trait Information {
    fn driverinfo_ltv(&self)-> String;
    fn driverinfo_htv(&self)-> String;
}

impl Driver{

    pub fn set_driver(name: String, experience: u8, ltype: String) -> Driver{
        Driver {
            name,
            experience,
            ltype
        }
    }
}

impl Information for Driver {

    fn driverinfo_ltv(&self)-> String
    {
        if self.ltype == "LTV".to_string(){
            format!("Name: {}\nExperience: {}\nLicense Type: {}",
                    self.name,
                    self.experience,
                    self.ltype              
                )
        }
        else{
            format!("Driving licesense not LTV")
        }
    }

    fn driverinfo_htv(&self)-> String
    {
        if self.ltype == "HTV".to_string() {
            format!("Name: {}\nExperience: {}\nLicense Type: {}",
                    self.name,
                    self.experience,
                    self.ltype
                )
        }
        else{
            format!("Driving licesense is not HTV")
        }
    }
}

fn main() {
    
    let driver_01 = Driver::set_driver(String::from("Nabeel"),10,String::from("LTV"));
    let driver_02 = Driver::set_driver(String::from("Faizan"),15,String::from("HTV"));

    println!("Driver 1\n{}\n",driver_01.driverinfo_ltv());
    //println!("Driver 1\n{}",driver_01.driverinfo_htv());
    println!("Driver 2\n{}",driver_02.driverinfo_htv());
    //println!("Driver 2\n{}",driver_02.driverinfo_ltv());


}
