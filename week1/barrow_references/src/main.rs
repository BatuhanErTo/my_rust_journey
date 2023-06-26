fn main() {
    let my_string = String::from("Hello world!");

    let ref_my_string_1 = &my_string;
    let ref_my_string_2 = &my_string;
    println!("First String : {}, Second String : {}", ref_my_string_1, ref_my_string_2);
    print_my_string(&my_string);

    let mut mut_my_string = String::from("Hello");
    append(&mut mut_my_string);
    println!("{}", mut_my_string);

    let original_value = String::from("HELLO WORLD!");
    let clone_original_value = original_value.clone();
    println!("Original Value : {}, Clone Value : {}", original_value, clone_original_value);

    println!("==> {}", copy_monster(&original_value));
}

fn copy_monster(value_to_cpd : &String) -> String{
    let mut value_to_return = value_to_cpd.clone();
    value_to_return.push_str(" MONSTER");
    value_to_return
}

fn append(s:&mut String){
    s.push_str(" world");
}

fn print_my_string(s:&String){
    println!("{}",s);
}