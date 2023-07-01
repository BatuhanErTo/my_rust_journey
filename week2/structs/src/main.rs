fn main() {
    let book = Book{
        title : "CASIO".to_string(),
        author : "EROL".to_string(),
        publication_year : 2002
    };

    println!("TITLE = {}, AUTHOR = {}, PUBLICATION YAER = {}",book.title, book.author, book.publication_year);

    let person = Person{
        name:String::from("Batuhan"),
        age:21
    };

    person.print();

    let mut person_mutable = Person{
        name:String::from("Batu"),
        age:12
    };

    person_mutable.age = 21;

    println!("name = {}, age = {}",person.name,person.age);
    println!("name = {}, age = {}",person_mutable.name,person_mutable.age);

    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
 
    println!("Alice's details: {:?}", alice);

    let point1 = Point{
        x:1.2,
        y:2.4
    };
    let point2 = Point{
        x:2.2,
        y:3.4
    };

    let distance:f64 = calculate_distance(point1,point2);
    println!("DISTANCE : {}",distance);

    let point3 = Point{
        x:1.2,
        y:2.4
    };
    let point4 = Point{
        x:2.2,
        y:3.4
    };

    let mid_point = midpoint(point3,point4);
    println!("MID POINTS = X : {}, Y : {}",mid_point.x, mid_point.y);

    let point = Point3D(1.0, 2.0, 3.0);
    println!("X : {}, Y : {}, Z : {}",point.0,point.1,point.2);

    let empty_struct = Empty;
    empty_struct.greet();

}

fn calculate_distance(p1:Point,p2:Point) -> f64{
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;
    (x_diff * x_diff + y_diff * y_diff).sqrt()
}

fn midpoint(p1: Point, p2: Point) -> Point {
    let x_mid = (p1.x + p2.x) / 2.0;
    let y_mid = (p1.y + p2.y) / 2.0;
    Point { x: x_mid, y: y_mid }
}


struct Book{
    title:String,
    author:String,
    publication_year:u32
}

#[derive(Debug)]
struct Person{
    name:String,
    age:u32
}

impl Person{
    fn print(&self){
        println!("{} : {}",self.name, self.age);
    }
}

struct Point{
    x:f64,
    y:f64
}

// TUPLE STRUCT
struct Point3D(f32, f32, f32);

// UNIT STRUCT
struct Empty;
impl Empty {
    fn greet(&self) {
        println!("Hello, I am an empty struct!");
    }
}
 