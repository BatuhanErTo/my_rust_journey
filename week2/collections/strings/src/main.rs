fn main() {
    let mut first_string = String::from("First String");
    let mut second_string = "Secondary String".to_string();

    first_string.push_str(" is awful!");

    println!("First modified String : {}",first_string);

    for chr in second_string.chars(){
        println!("{chr}");
    }

    for byte in second_string.bytes(){
        println!("{byte}");
    }
}

