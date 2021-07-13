use std::{mem, usize};

// part 1 - learning rust
fn fundamental_datatypes() {
    println!("\n *** FUNDAMENTAL DATATYPES ***");
    // immutables
    println!("\nImmutable values");
    let an_unsigned_height_bits_variable:u8 = 123;      // 0 - 255
    let a_signed_height_bits_variable:i8 = -123;        // -128 - 127
    println!("-> an unsigned height bits variable = {}", an_unsigned_height_bits_variable);
    println!("-> a signed height bits variable = {}", a_signed_height_bits_variable);

    // mutables
    println!("\nMutable values");
    let mut a_mutable_value:i8 = -123;
    println!("-> (before change) a mutable value = {}", a_mutable_value);
    a_mutable_value = -128;
    println!("-> (after change) a mutable value = {}", a_mutable_value);

    // practice
    println!("\nPractice");
    let mut c = 1234566789;     // 32-bit signed i32
    println!("-> (before change) c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("-> (after change) c = {}, size = {} bytes", c, mem::size_of_val(&c));
    
    let uz:usize = 123;
    let size_of_uz = mem::size_of_val(&uz);
    let iz:isize = -123;
    let size_of_iz = mem::size_of_val(&iz);
    println!("-> uz = {}, takes up {} bytes, {} bit-OS", uz, size_of_uz, size_of_uz * 8);
    println!("-> iz = {}, takes up {} bytes, {} bit-OS", iz, size_of_iz, size_of_iz * 8);

    // chars
    println!("\nCharacters");
    let a_character = 'x';
    println!("-> a character = {}, size = {} bytes", a_character, mem::size_of_val(&a_character));

    // floats
    println!("\nFloats");
    let a_float = 2.5;      // double precision, 8 bytes or 64 bits, f64
    println!("-> a float = {}, size = {} bytes", a_float, mem::size_of_val(&a_float));

    // booleans
    println!("\nBooleans");
    let a_boolean = false;
    println!("-> a boolean = {}, size = {} bytes", a_boolean, mem::size_of_val(&a_boolean));
}

// part 2 - learning rust
fn operators() {
    println!("\n *** OPERATORS ***");
    // arithmetic
    let mut a = 2 + 3 * 4;
    let mut b = 7;
    println!("a = {}", a);
    println!("b = {}", b);

    a = a + 1;      // -- and ++ don't work
    a -= 2;         // +=, -=, *=, /= and %= works
    b /= 2;

    let value_of_a_divided_by_b = a / b;
    let remainder_of_a_divided_by_b = a % b;
    println!("-> {} / {} is equal to {} with a remainder of {}", a, b, value_of_a_divided_by_b, remainder_of_a_divided_by_b);

    let a_cubed = i32::pow(a, 3);
    println!("-> {} cubed is equal to {}", a, a_cubed);

    let c = 2.5;
    println!("c = {}", c);
    let c_cubed = f64::powi(c, 3);
    let c_to_pi = f64::powf(c, std::f64::consts::PI);
    println!("-> {} cubed is equal to {}", c, c_cubed);
    println!("-> {} to power of PI is equal to {}", c, c_to_pi);

    // bitwise
    let d = 1 | 2;      // | OR, & AND, ^ XOR and ! NOR
                            // 01 | 10 = 11 == 3_10
    println!("-> 1 | 2 = {}", d);

    let two_to_10 = 1 << 10;
    println!("-> 2^10 = {}", two_to_10);

    let pi_less_4 = std::f64::consts::PI < 4.;
    println!("-> PI < 4 = {}", pi_less_4);

    let x = 5;
    let x_is_5 = x == 5;
    println!("-> {} == 5 = {}", x, x_is_5);
}

// part 3 - learning rust
fn scope_and_shadowing() {
    println!("\n *** SCOPE AND SHADOWING ***");

    let a = 123;
    println!("-> (outside scope) a = {}", a);

    {
        let b = 256;
        println!("-> (inside scope) b = {}", b);
        let a = 257;
        println!("-> (inside scope) a = {}", a);
        
    }

    println!("-> (outside scope) a = {}", a);
}

// part 4 - learning rust
const MEANING_OF_LIFE:u8 = 42;
static HELL:u16 = 666;
static mut WRLD:u16 = 666;

fn constants() {
    println!("\n *** CONSTANTS ***");

    println!("-> a global variable : {}", MEANING_OF_LIFE);

    // use of statics is not recommended, better use constants
    println!("-> a static global variable : {}", HELL);
    unsafe {
        println!("-> a static mutables global variable : {}", WRLD);
        WRLD = 999;
        println!("-> a static mutables global variable : {}", WRLD);
    }
}

// part 3 - learning rust
// TODO set it into models/point and import
struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point {x: 0.0, y: 0.0}
}

fn stack_and_heap() {
    println!("\n *** STACK AND HEAP ***");

    // the STACK is the SHORT-TERM memory in the RAM
    println!("the STACK is the SHORT-TERM memory in the RAM");
    let stack_x = 5;
    let stack_y = 3200;
    println!("-> x = {} is in the stack, memory adress is {:p}", stack_x, &stack_x);
    println!("-> y = {} is in the stack, memory adress is {:p}", stack_y, &stack_y);

    // the HEAP is the LONG-TERM memory in the RAM
    println!("the HEAP is the LONG-TERM memory in the RAM");
    let stack_z = Box::new(10);
    println!("-> z = {} is in the stack, its memory adress is {:p}
        but it is pointing to the heap : its value from the heap is {}, located at {:p}", *stack_z, &stack_z, *stack_z, &*stack_z);

    // demo
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 takes up {} bytes and is in the stack", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes and is pointing to the heap", mem::size_of_val(&p2));
    let p3 = *p2;
    println!("p3 = {} takes up {} bytes and is in the stack", p3.x, mem::size_of_val(&p3));
}

fn main() {
    println!("--- TYPES AND VARIABLES ---");
    
    fundamental_datatypes();
    operators();
    scope_and_shadowing();
    constants();
    stack_and_heap();
}