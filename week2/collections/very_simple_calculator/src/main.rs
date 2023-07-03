fn main() {
    let divide = Mod::Divide{a :1.0, b : 0.0};
    let forth = divide.calculate();

   match forth{
        Ok(result) => println!("RESULT OF DIVISON IS : {}",result),
        Err(message) => println!("{message}")
   }
}

enum Mod{
    Summation{a: f64, b: f64},
    Subtraction{a: f64, b: f64},
    Multiplication{a: f64, b: f64},
    Divide{a: f64, b: f64}
}


impl Mod{
    fn calculate(&self) -> Result<f64,String>{
        match self{
            Mod::Summation{a,b} => {Ok(a+b)},
            Mod::Subtraction{a,b} => {Ok(a-b)},
            Mod::Multiplication{a,b} => {Ok(a*b)},
            Mod::Divide{a,b} =>{
                if b <= &0.0{
                    Err("Deminator cannot be zero or less!!".to_string())
                }else{
                    Ok(a/b)
                }
            }
        }
    } 
}