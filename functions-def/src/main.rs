
// high order functions -
fn high_order_functions() {
    println!("*** HIGH ORDER FUNCTIONS ***");

}

// closures -
fn say_hello(name:&str) { println!("Hello {}! :-)", name); }

fn closures() {
    println!("*** CLOSURES ***");

    say_hello("Juice");

    let greetings = say_hello;
    greetings("Jey Laylow");

    // between |...| are arguments
    // after -> is return type
    // between {...} is the function
    // -> creates a function only accessible from the code part where it has been created
    let plus_one = |x:i32| -> i32 { x + 1 };

    let a = 998;
    println!("{} + 1 = {}", a, plus_one(a));

    let two = 2;
    let plus_two = |x| {
        let mut z = x;
        z += two;
        z
    };
    println!("{} + 2 = {}", a, plus_two(a));

    // T : by value
    // T& : by reference
    // &mut T : by mutable reference
    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    println!("f before change = {} at address {:p}", f, &f);
    plus_three(&mut f);
    println!("f after change = {}", f);
}

// methods -
struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn methods() {
    println!("*** METHODS ***");

    let p1 = Point { x: 3.4, y: 4.9 };
    let p2 = Point { x: 1.2, y: 7.4 };
    let line = Line { start: p1, end: p2 };

    println!("the length of the line is {}", line.len());
}

// functions and arguments -
fn print_value(x:i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    println!("-> the argument's address is {:p} with x pointing on it from address {:p}", &*x, &x);
    *x += 1;
}

fn product(x:i32, y:i32) -> i32 {
    x * y   // no semicolon at the end allows the return
    // -> equivalent to `return x * y;`
}

fn functions_and_args() {
    println!("*** FUNCTIONS AND ARGUMENTS ***");

    print_value(32);

    let mut z = 1;
    println!("z before change = {} at address {:p}", z, &z);
    increase(&mut z);
    println!("z after change = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

fn main() {
    println!("--- FUNCTIONS ---");

    // functions_and_args();
    // methods();
    closures();
}
