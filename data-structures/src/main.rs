use std::mem;

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

    let cRgb:Color = Color::RgbColor(0, 134, 168);
    let cCmyk = Color::Cmyk{ cyan: 0, magenta: 128, yellow: 92, black: 255 };

    color_description(cRgb);
    color_description(cCmyk);   
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

    // structures();
    // enumerations();
    // option();
    arrays();
}
