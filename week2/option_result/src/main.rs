fn main() {
    let number = 4.0;
    let root_of_number = find_square_root(number);

    match root_of_number{
        Some(value) => println!("Root of the given number is : {}",value),
        None => println!("The square root of {} is not a real number.", number)
    }

    let a = 2.6;
    let b = 0.0;
    let division_result = divide(a,b);

    match division_result{
        Ok(value) =>  println!("Value of divison is {}", value),
        Err(message) => println!("{message}")
    }

    let base = get_from_db("base");
    let height = get_from_db("height");
    let area_result = calculate_triangle_area(base, height);

    match area_result {
        Ok(area) => println!("The area of the triangle is: {} square units.", area),
        Err(error_message) => println!("Error: {}", error_message),
    }
}

fn find_square_root(number : f64) -> Option<f64>{
    if number >= 0.0{
        Some(number.sqrt())
    }else{
        None
    }
}

fn divide(a : f64, b : f64) -> Result<f64, String>{
    if b == 0.0{
        Err("Division by zero is not allowed".to_string())
    }else{
        Ok(a/b)
    }
}

fn get_from_db(key : &str) -> Option<f64>{
    let database : [(&str,Option<f64>);2] = [
        ("base", Some(4.0)),
        ("height", Some(6.0)),
    ];

    for (k,v) in database{
        if k == key{
            return v;
        }
    }
    None
}

fn calculate_triangle_area(base : Option<f64>, height : Option<f64>) -> Result<f64,String>{
    match (base,height){
        (Some(b),Some(h)) => {
            if b <= 0.0 || h <= 0.0{
                Err("Base and hight must be positive numbers".to_string())
            }else{
                Ok(b*h*0.5)
            }
        },
        (None, _) => Err("The base is missing.".to_string()),
        (_, None) => Err("The height is missing.".to_string()),
    }
}