fn main() {
    let mut pointer = Box::new(5);
    println!("{}",pointer);
    *pointer = 10;
    println!("{}",pointer);

    let trait_object: Box<dyn MakeNoise> = Box::new(Bird{
        name : "Lemon".to_string(),
        color : "Yellow".to_string()
    });

    trait_object.talk();

    invite_animal_to_stage(trait_object);

    let mut speakers: Vec<Box<dyn MakeNoise>> = Vec::new();
    speakers.push(Box::new(Bird{name:"XYZ".to_string(),color:"Green".to_string()}));
    speakers.push(Box::new(Dog{name:"XDOG".to_string(),charm_point:10}));
    for speaker in speakers{
        speaker.talk();
    }
}

trait MakeNoise{
    fn talk(&self);
}

struct Bird{
    name: String,
    color: String
}

struct Dog{
    name: String,
    charm_point: i32
}

impl MakeNoise for Bird{
    fn talk(&self){
        println!("Birdy is talking");
    }
}

impl MakeNoise for Dog{
    fn talk(&self){
        println!("Dog is talking");
    }
}

fn invite_animal_to_stage(speaker: Box<dyn MakeNoise>){
    println!("Here it comes...");
    speaker.talk();
}
