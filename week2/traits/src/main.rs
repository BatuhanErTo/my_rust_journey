fn main() {
    let dog = Dog{
        name : "KARABAS".to_string(),
    };

    let cat = Cat{
        name : "SANSLI".to_string(),
    };


    cat.speak();
    dog.speak();

    let original_string = String::from("XX-XX-YY");
    let copy_string = original_string.display();
    println!("{}",copy_string);

    animal_speak(&cat);
    let monkey = Monkey;
    monkey.make_sound();
    monkey.walk();
    monkey.sleep();
    
}

trait Speak{
    fn speak(&self);
}

struct Dog{
    name : String
}

struct Cat{
    name : String
}

trait Display{
    fn display(&self) -> String;
}

impl Display for String{
    fn display(&self) -> String{
        self.clone()
    }
}

impl Speak for Dog{
    fn speak(&self){
        println!("HAV HAV : {}",self.name);
    }
}

impl Speak for Cat{
    fn speak(&self){
        println!("MIYAV MIYAV : {}",self.name);
    }
}

fn animal_speak<T : Speak>(animal : &T){
    animal.speak();
}

trait Animal{
    fn make_sound(&self);
    fn sleep(&self){
        println!("Have some rest");
    }
}

trait Mammal : Animal{
    fn walk(&self);
}

trait Bird : Animal{
    fn fly(&self);
}

struct Monkey;

impl Animal for Monkey{
    fn make_sound(&self){
        println!("I am a Monkey");
    }
}

impl Mammal for Monkey{
    fn walk(&self){
        println!("I've got legs");
    }
}