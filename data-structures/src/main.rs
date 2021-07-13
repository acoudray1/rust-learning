use std::mem;

// generics -
struct GenericPoint<T> {
    x: T,
    y: T
}

struct GenericPointTwo<T,V> {
    x: T,
    y: V
}

struct GenericLine<T> {
    start: GenericPoint<T>,
    end: GenericPoint<T>
}

fn generics() {
    println!("*** GENERICS ***");

    let gp1:GenericPoint<f64> = GenericPoint{ x: 1.2, y: 2.6 };
    let gp11:GenericPoint<f64> = GenericPoint{ x: 0.2, y: 3.6 };
    let gp2:GenericPoint<i32> = GenericPoint{ x: 1, y: 2 };
    let gp3:GenericPointTwo<f64,u16> = GenericPointTwo{ x: 1.2, y: 0 };
    let gp4:GenericPointTwo<i32,f64> = GenericPointTwo{ x: 1, y: 2.23 };

    let line = GenericLine{ start: gp1, end: gp11 };
}

// pattern_matching -
fn how_many(x:i32) -> &'static str {
    return match x {
        0 => "no",
        1 | 2 => "one or two",
        9..=11 => "lots of",
        12 => "a dozen",
        _ if x % 2 == 0 => "some",
        _ => "a few"
    }
}

fn where_coordinates(point:(u8,u8)) -> String {
    return match point {
        (0,0) => "origin".to_string(),
        (0,y) => format!("x axis with y = {}", y),
        (x,0) => format!("x = {} on the y axis", x),
        // (ref mut x,0) => format!("x = {} on the y axis", x), // allows to modify by reference x
        (x,y) => format!("({}, {})", x, y)
    }
}

fn color_description_with_pattern_matching(color: Color) {
    match color {
        Color::Red                                                        => println!("r"),
        Color::Blue                                                       => println!("g"),
        Color::Green                                                      => println!("b"),
        Color::RgbColor(0, 0, 0)
            | Color::Cmyk{ black: 255, .. }    => println!("black!"),
        Color::RgbColor(r, g, b)                                          => println!("rgb({}, {}, {})", r, g, b),
        _                                                                 => println!("Unkown color")
    }
}

fn pattern_matching() {
    println!("*** PATTERN MATCHING ***");

    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (0, 0);
    println!("the point is located on {}", where_coordinates(point));

    let point2 = (0, 4);
    println!("the second point is located on {}", where_coordinates(point2));

    let c_rgb:Color = Color::RgbColor(0, 134, 168);
    let c_cmyk = Color::Cmyk{ cyan: 0, magenta: 128, yellow: 92, black: 255 };
    color_description(c_rgb);
    color_description(c_cmyk); 
}

// tupples -
fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
    return (x + y, x * y);
}

fn tupples() {
    println!("*** TUPPLES ***");

    let x = 3;
    let y = 4;

    let tupple = sum_and_product(x, y);

    println!("tupple = {:?}", tupple);
    println!("({0} + {1} = {2}, {0} * {1} = {3})", x, y, tupple.0, tupple.1);

    // destructuring
    let (a,b) = tupple;
    println!("a = {}, b = {}", a, b);

    let tupple2 = sum_and_product(7, 4);
    let combined_tupple = (tupple, tupple2);
    println!("combined_tupple = {:?}", combined_tupple);
    println!("last element of combined_tupple is {}", combined_tupple.1.1);

    let ((c,d), (e,f)) = combined_tupple;
    println!("c = {}, d = {}, e = {}, f = {}", c, d, e, f);

    // multiple typesn and elements
    let foo = (true, 42.0, -1i8);
    println!("foo = {:?}", foo);

    // only one element
    let meaning = (42,);
    println!("meaning = {:?}", meaning);
}

// strings -
fn strings() {
    println!("*** STRINGS ***");

    // static means that it will be referenced in our program
    // reassignation doesn't work
    // utf-8
    let s:&'static str = "99 problems";     //&str = string slice

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(c) = s.chars().nth(0) {
        println!("First character is \'{}\'!", c);
    }
    
    // heap - string
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        if (a as char) != ('z') {
            // letters.push(',');
            letters.push_str(", ");
        }
        a += 1;
    }
    println!("letters = ({}) with size of {} bytes located at {:p}", letters, mem::size_of_val(&letters), &letters);

    // &str <> String
    let u:&str = &letters;
    println!("u = ({}) with size of {} bytes located at {:p}", u, mem::size_of_val(&u), &u);

    // concatenation
    // String + str
    let v = letters + "abc";
    let mut w = String::from("armed and dangerous");
    let mut x = "aicyp666".to_string();
    println!("{}", x);
    println!("taking the evil and making it good... {}", x.replace("666", "999"));
}

// slices - 
fn use_slice(slice:&mut[i32]) {
    println!("first element = {} and length of slice is {}", slice[0], slice.len());
    slice[0] = 999;
}

fn slices() {
    println!("*** SLICES ***");

    let mut data = [1, 2, 3, 4, 5];

    println!("data = {:?}", data);
    use_slice(&mut data[1..4]);
    println!("data = {:?}", data);
    use_slice(&mut data);
    println!("data = {:?}", data);
}

// vectors -
fn vectors() {
    println!("*** VECTORS ***");

    let mut veca = Vec::new();
    veca.push(1);
    veca.push(2);
    veca.push(3);
    println!("veca : {:?}, length : {}", veca, veca.len());

    veca.push(4);
    println!("veca : {:?}, length : {}, and its first value is veca[0] = {}", veca, veca.len(), veca[0]);

    let i:usize = 3;
    veca[i] = 333;
    println!("veca[{}] = {}", i, veca[i]);
    
    // option
    match veca.get(7) {
        Some(x) => println!("veca[7] = {}", x),
        None => println!("error, no such element !")
    }

    veca.push(34);
    veca.push(777);
    for x in &veca {
        println!("{}", x);
    }
    println!("veca = {:?}", veca);

    // what's poppin
    let last_elem = veca.pop();
    println!("{:?}", last_elem);
    match last_elem {
        Some(x) => println!("the last element value is {} and it has been removed from veca", x),
        None => println!("error, no element was popped...")
    }
    println!("veca = {:?}", veca);

    let mut popopop = String::from("i");
    // while and vectors
    while let Some(x) = veca.pop() {
        println!("what's popp{}n ?? {}", popopop, x);
        popopop.push('i');
    }
    println!("veca = {:?}", veca);
}

// arrays
fn arrays() {
    println!("*** ARRAYS ***");

    let mut a:[i32; 5] = [1, 2, 3, 4, 5];

    println!("a has {} elements and first is equal to {}", a.len(), a[0]);
    a[0] = 321;
    println!("after modification, a[0] = {}", a[0]);
    println!("a = {:?}", a);    // debug mode {:?}

    if a != [1, 2, 3, 4, 5] {
        println!("No match with 1..=5 array...");
    }

    let b:[u8; 10] = [1; 10];
    println!("b = {:?}", b);
    for x in 0..b.len() {
        println!("{} : {}", x, b[x]);
    }
    println!("b took up {} bytes", mem::size_of_val(&b));

    let c = [1u16; 10];
    println!("c = {:?}", c);
    for x in 0..c.len() {
        println!("{} : {}", x, c[x]);
    }
    println!("b took up {} bytes", mem::size_of_val(&c));

    let matrix:[[f32;3]; 2] = 
        [
            [1.0, 0.0, 0.0],
            [0.0, 2.0, 0.0]
        ]; // matrix of 2 rows by 3 columns
    println!("matrix = {:?}", matrix);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == j {
                println!("matrix[{}][{}] = {}", i, j, matrix[i][j]);
            }
        }
    }
}

// option -
fn option() {
    println!("*** OPTION ***");

    let x = 3.0;
    let y = 2.0;

    // Option = Some(z) or None
    let result:Option<f64> = 
        if y != 0.0 { Some(x/y) } else { None };

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("We cannot divide {} by {}", x, y)
    }

    // if let / while let
    // destructuring, if it fails nothing happens
    if let Some(z) = result { println!("z = {}", z); }
}

// enumerations - 
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    Cmyk { cyan: u8, magenta: u8, yellow: u8, black: u8 }, // struct
}

fn color_description(color: Color) {
    match color {
        Color::Red                                                        => println!("r"),
        Color::Blue                                                       => println!("g"),
        Color::Green                                                      => println!("b"),
        Color::RgbColor(0, 0, 0)
            | Color::Cmyk{ cyan: _, magenta: _, yellow: _, black: 255}    => println!("black!"),
        Color::RgbColor(r, g, b)                                          => println!("rgb({}, {}, {})", r, g, b),
        _                                                                 => println!("Unkown color")
    }
}

fn enumerations() {
    println!("*** ENUMERATIONS ***");

    let c_rgb:Color = Color::RgbColor(0, 134, 168);
    let c_cmyk = Color::Cmyk{ cyan: 0, magenta: 128, yellow: 92, black: 255 };

    color_description(c_rgb);
    color_description(c_cmyk);   
}

// structures - 
struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    println!("*** STRUCTURES ***");

    let p1 = Point { x: 3.4, y: 4.9 };
    println!("point p1 is at ({}, {})", p1.x, p1.y);

    let p2 = Point { x: 1.2, y: 7.4 };
    println!("point p2 is at ({}, {})", p2.x, p2.y);

    let line = Line { start: p1, end: p2 };
    println!("line is going from point ({}, {}) to point ({}, {})", line.start.x, line.start.y, line.end.x, line.end.y)
}

fn main() {
    println!("--- DATA STRUCTURES ---");

    structures();
    enumerations();
    option();
    arrays();
    vectors();
    slices();
    strings();
    tupples();
    pattern_matching();
    generics();
}
