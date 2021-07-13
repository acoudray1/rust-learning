
// if_statement - 
fn if_statement() {
    println!("*** IF STATEMENT ***");
    let temperature = 30;

    if temperature > 35 {
        println!("It's sooo hot outside !");
    } else if temperature < 0 {
        println!("It's really cold outside !");
    }
    else {
        println!("Temperature is okay");
    }

    let day = if temperature > 20 { "sunny" } else if temperature < 0 { "snowy" } else { "cloudy" };
    println!("Today is {}", day);

    println!("it is {}",
        if temperature > 20 {
            if temperature > 30 { "very hot" } else { "hot" }
        } else if temperature < 10 { 
            if temperature > 0 { "very cold" } else { "cold" }
        } else { "okay" });
}

// while_and_loop - 
fn while_and_loop() {
    println!("*** WHILE AND LOOP ***");
    let mut x = 1;
    let mut y = 1;

    // condition to break-out
    while x < 1000 {
        x *= 2;

        if x == 64 { println!("continue!"); continue; }

        println!("x = {}", x);
    }

    // eq to while-true
    loop {
        y *= 2;

        println!("y = {}", y);

        if y == 1 << 10 { println!("break!"); break; }
    }
}

// for_loop -
fn for_loop() {
    println!("*** FOR LOOP ***");

    for x in 1..11 {        // excludes 11
        if x == 3 { println!("continue!"); continue; }

        println!("x = {}", x);

        if x == 8 { println!("break!"); break; }
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{} : {}", pos, y);
    }
}

// match_statement -
fn match_statement() {
    println!("*** MATCH STATEMENT ***");

    let country_code = 999;

    let country = match country_code {
        44       => "United Kingdom",
        33       => "France",
        46       => "Sweden",
        7        => "Russia",
        1...999  => "unkown",   // includes 999
        _        => "invalid"
    };

    println!("the country with code {} is {}", country_code, country);
}

fn main() {
    println!("--- CONTROL FLOW ---");

    // if_statement();
    // while_and_loop();
    // for_loop();
    match_statement();
}
