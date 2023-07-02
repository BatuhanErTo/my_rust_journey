fn main() {
    let _current_weather = Weahter::Sunny;
    let message = Message::ChangeColor(-1,2,4);
    message.print_value();

    if let Message::ChangeColor(r,g,b) = message{
        println!("CHANGE COLOR R : {}, G : {}, B : {}",r,g,b);
    } else{
        println!("Something different");
    }
}

enum Weahter{
    Sunny,
    Cloudy,
    Rainy,
    Snowy
}

enum Message{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn print_value(&self){
        match self{
            Message::Quit => println!("QUIT"),
            Message::Move{x,y} => println!("MOVE X : {}, Y : {}",x,y),
            Message::Write(message) => println!("WRITE : {}",message),
            Message::ChangeColor(r,g,b) => println!("CHANGE COLOR R : {}, G : {}, B : {}",r,g,b)
        }
    }
}