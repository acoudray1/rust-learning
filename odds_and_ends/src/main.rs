
// building modules and crates -
extern crate odds_and_ends_lib;
use odds_and_ends_lib::greetings::french;

fn building_modules_and_crates() {
    println!("*** BUILDING MODULES AND CRATES ***");

    // go to odds_and_ends_lib to see how to create a lib / module
    // look at .toml to see how to import it to project
    println!("English: {}, {}",
        odds_and_ends_lib::greetings::english::hello(),
        odds_and_ends_lib::greetings::english::bye());

    println!("French: {}, {}", french::hello(), french::bye());
}

// consuming crates -
extern crate rand;
use rand::Rng;

fn consuming_crates() {
    println!("*** CONSUMING CRATES ***");
    // go to crates.io, find you package and add it to the .toml
    // -> run `cargo build`

    let mut rng = rand::thread_rng();

    let randomBool:bool = rng.gen();
    let randomNumber:i32 = rng.gen();
    println!("randomBool = {}", randomBool);
    println!("randomNumber = {}", randomNumber);
}

fn main() {
    println!("--- ODDS AND ENDS ---");

    consuming_crates();
    building_modules_and_crates();
}
