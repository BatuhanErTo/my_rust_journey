fn main() {
    let _message = "hello! Imma batu";
    let _x: i32 = 42;
    let _pi: f64 = 3.14159;
    let _is_rust_fun: bool = true;
    let _letter_a: char = 'a';

    let x = 45;

    if x > 0{
        println!("x is non-negative");
    }else {
        println!("x is negative");
    }

    let mut i = 1;
    while i <= 5 {
        println!("{}",i);
        i+=1;
    }

    // 8,16,64,128
    let _x: i8 = 12;
    let _y: i64 = 1221;
    let _z: u64 = 1243134;

    // 32, 64
    let _num1: f32 = 12.5;

    let _message: &str = "Hello, world!";
    let _name = String::from("Alice");

    let _day_of_the_week: [&str;7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday"
    ];

    let _first_day = _day_of_the_week[0];

    // Slices

    let _first_two_Days = &_day_of_the_week[0..3];

    // Tuples
    let person = ("Alice", 30);
    let name = person.0;
    let age = person.1;
    println!("The person's name is {} and their age is {}.", name, age);

    // mutable

    let mut _mutable_value = 2134;
    _mutable_value = 31;

    let _number = add(42, 23);
    let number1 = ret_1();

    println!("{}",number1);

    let mut counter = 0;

    while counter <= 5{
        println!("WHILE COUNTER : {}",counter);
        counter += 1;
    }

    let numbers: [i32;5] = [1,2,3,4,5];

    for number in numbers{
        println!("FOR NUMBER : {}",number)
    }

    for number in 0..=5{
        println!("SECOND FOR NUMBER : {}",number);
    }
    
    counter = 0;
    loop{
        println!("Learnin sth new is not bad");
        counter += 1;
        if counter == 2{
            break;
        }
    }

    let check_number:i8 = 1;
    match check_number{
        1 => {
            println!("NUmber is 1");
            let funny = 8;
            println!("Funny number {}",funny);
        }
        2 => println!("GOodbye life"),
        _ => println!("SOMetimes there is no escape from karma!"),
    }

    let gonna_match = match check_number{
        1 => "GOnna match is 1",
        2 => "GOnna match must go",
        _ => "could go somewhere else",
    };

    println!("LAst statement bu judge :: {}", gonna_match);
}

fn add(x:i32, y:i32) -> i32{
    x+y
}

fn ret_1() -> i32{
    println!("Return 1");
    1
}

