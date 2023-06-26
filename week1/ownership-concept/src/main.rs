fn main() {
    let first_owner = String::from("TO-DO");
    let mut second_owner = first_owner;
    println!("OUTPUT : {}",second_owner);
    second_owner.push_str("-Not-To-Do");
    println!("OUTPUT : {}",second_owner);

    let slice = &second_owner[2..4];
    println!("{}",slice);
}
