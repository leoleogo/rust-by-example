#![allow(unreachable_code)]

pub fn test_foc() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    //loop
    let mut count = 0u32;
    loop {
        count += 1;
        if count == 3 {
            println!("couter is {}", count);
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("Ok that's enough");   
            break;
        }
    }

    'outer: loop {
        println!("Outer loop");
        'inner: loop {
            println!("entered inner");
            break 'outer;
        }
        println!("never reached");
    }
    println!("Exited the outer loop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    //while
    //FizzBuzz
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0{
            println!("fizz");
        } else if  n % 5 == 0 {
            println!("buzz");   
        } else {
            println!("{}", n);
        }
        n += 1;
    }

    //for
    //FizzBuzz
    for n in 1..=101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    //Iterator
    let mut names = vec!["Bod", "Leo", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Leo" => println!("There is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }
    println!("names: {:?}", names);
    for name in names.iter_mut() {
        match name {
            &mut "Leo" => println!("There is a rustacean among us!"),
            _ => *name = "hello",
        }
    }
    println!("names: {:?}", names);

    let number = 11;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime: {}", number),
        // Match an inclusive range
        a @ 13..=19 => println!("A teen of {}", a),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);

    //destructured
    let pair = (0, -2);
    println!("tell me about {:?}", pair);
    match pair {
        (0, y) => println!("First is 0, and y is {:?}", y),
        (x, 0) => println!("First is {}, and y is 0", x),
        _ => println!("other"),
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }
    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }
    //pointers ref
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_ref = 3;
    let ref _is_a_ref = 4;
    let value = 5;
    let mut mut_value = 4;
    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y);
    }

    let pair = (2, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    fn age() -> u32 {
        15   
    }

    match age() {
        0 => println!("I'am not born yet I guess"),
        n @ 1 ..= 12 => println!("I'am a child of age {:?}", n),
        n => println!("I'am a person of age {}", n),
    }

     // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    enum Foo1 {
        Bar,
        Baz,
        Qux(u32),
    }

    // Create example variables
    let a = Foo1::Bar;
    let b = Foo1::Baz;
    let c = Foo1::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo1::Bar = a {
        println!("a is foobar");
    }
    
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo1::Bar = b {
        println!("b is foobar");
    }
    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo1::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo1::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}