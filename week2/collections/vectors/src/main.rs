fn main() {
    let mut numbers = vec![1,2,3,4,5];
    for number in &numbers{
        println!("i : {}",number);
    }
    let slice = &numbers[1..4];
    for number in slice{
        println!("i : {}",number);
    }
    let mut names : Vec<String> = Vec::new();
    names.push(String::from("Batuhan"));
    names.push(String::from("Ahmet"));
    println!("Firts name of the vector : {}",&names[0]);
}
