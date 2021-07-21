
// borrowing -
fn borrowing() {
    println!("*** BORROWING ***");

    let print_vector = |x:&Vec<i32>| {
        let mut j = 0;
        for i in x {
            println!("x[{}] = {}", j, i);
            j += 1;
        }
        // x.push(123)     // can't work bc x is immutable reference
        println!("{:?}", x);
    };

    let v = vec![3, 2, 1];
    print_vector(&v);
    for i in 0..=2 {
        println!("v[{}] = {}", i, v[i]);
    }
    println!("{:?}", v);

    let mut a = 40;
    println!("a = {}", a);
    let b = &mut a;
    *b += 2;        // accessing the reference's values
    println!("(after mutable ref change) -> a = {}", a);

    let mut z = vec![9, 8, 7];
    for i in &z {
        println!("i = {}", i);
    }
}

// ownership -
fn ownership() {
    println!("*** OWNERSHIP ***");

    // v is on the stack
    // the data are on the heap
    let v = vec![1, 2, 3];

    // v2 points to the same content as v in the heap
    // -> in rust v is no longer usable because to values cant point to the same data
    let v2 = v;

    // println!("{:?}", v);     // can't work
    println!("{:?}", v2); 

    let foo = |v:Vec<i32>| ();
    foo(v2);
    // println!("{:?}", v2);    // can't work

    let u = 1;
    let u2 = u;
    println!("u = {}", u);      // working bc u:i32 implements the copy traits

    let w = Box::new(1);
    let w2 = w;
    // println!("w = {}", w);      // can't work bc w pointing to the heap so value cannot be copied
    println!("w2 = {}", w2);

    let vvs = vec![1, 2, 3];
    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    let vv = print_vector(vvs);     // we take ownership of vvs vector and return it to vv
    println!("{}", vv[0]);
}

fn main() {
    println!("--- LIFETIME ---");

    ownership();
    borrowing();
}
