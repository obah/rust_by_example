enum Temperature {
    Celcius(i32),
    Fahrenheit(i32),
}

fn regular_loop() -> u32 {
    let mut count: u32 = 0;

    println!("Starting loop");

    loop {
        count += 1;

        //the 5 break will be skipped becuase of the continue at 5 then 10 triggers
        if count == 5 {
            println!("count 5 is skipped");

            continue;
        }

        if count == 5 {
            break count * 5;
        }

        if count == 10 {
            break count.pow(2);
        }

        println!("count is {count}");
    }
}

pub fn get_loop() {
    let count_val = regular_loop();
    println!("The count evaluated to {count_val}");
}

pub fn nested_loop() {
    'outer: loop {
        println!("Started the outer loop");

        'inner: loop {
            println!("entered the inner loop");

            // break; //this breaks the inner loop

            break 'outer; //breaks the outer loop and hence the entire loop
        }

        println!("This code is never reached");
    }

    println!("Ended the outer loop");
}

//fizzbuzz
pub fn fizzbuzz() {
    let mut count = 1;

    while count < 101 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{count}");
        }

        count += 1;
    }
}

pub fn for_loop() {
    let names = ["oba", "todimu", "emmanuel"];

    for name in names.iter() {
        println!("{name}");
    }

    println!("names: {:?}", names);
}

fn age() -> u32 {
    15
}

pub fn match_loop(num: u32, boolean: bool) {
    println!("Facts about the number {num}: ");

    match num {
        0 => println!("Zero!!!!"),
        2 | 3 | 5 | 7 => println!("Prime number lesser than 10"),
        13..19 => println!("A teen"),
        _ => println!("Ugh! nothing special"),
    }

    let binary = match boolean {
        true => 1,
        false => 0,
    };

    println!("The current binary value is {binary}");

    //match tuple
    let triple = (3, -12, 3);
    println!("Facts about the tuple {:?}: ", triple);

    match triple {
        (0, y, z) => println!("First is 0, second is {:?} and last is {:?}", y, z),
        (1, ..) => println!("First is 1 and the rest dont matter"),
        (.., 2) => println!("Last is 2 and the rest dont matter"),
        (3, .., 4) => println!("First is 3, last is 4 and the rest dont matter"),
        _ => println!("Ugh! Nothing special"),
    }
    //same destructuring works with arrays and slices, _ ignores 1, .. ignores mutiple
    //matching references and pointers are handled differently also

    //guards on enum matching
    let temperature = Temperature::Celcius(32);

    match temperature {
        //here's the guard with the if
        Temperature::Celcius(t) if t > 30 => println!("Temperature is too high"),
        Temperature::Celcius(s) => println!("Temperature is {s}c"),
        Temperature::Fahrenheit(x) if x < 86 => println!("Temperature is too cold"),
        Temperature::Fahrenheit(y) => println!("Dont care, use celcius"),
    }

    //binding
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }
}

pub fn if_let() {
    let num = Some(10);

    if let Some(i) = num {
        println!("Matched num: {:?}", i);
    }
}

//let-else and while-let also exist
