use std::io;

fn main() {
    println!("Hello, world!");  // ! means macro
    let x = 4; // compiler determines type at runtime. By default x is immutable, either use mut or redeclare. Re-declaring with let can change type
    // let mut y  = 5;
    println!("x is: {}", x);

    {
        let x = x - 2;
        println!("x is: {}", x); // prints 2. "Name shadowing"... x from the outer scope is used in declaration of x in the inner scope.
        let x = 2;
        println!("x is: {}", x); // prints 2
    }

    let x = x + 1; // last declaration can be used in new declaration (with let)
    println!("x is: {}", x);

    //constants. Value and type cannot change throughout program
    const SECONDS_IN_MINUTE: u32 = 60; // const must have a type
    println!("{}", SECONDS_IN_MINUTE);


    // data types
    let integer: i32 = 2; // i32 == int 32bit, u32 == unsigned int 32bit, u8 and i8 also exist.
    let floating_point: f32 = 10.90; // f32, f64 (default) -- floats (single, double precision)
    let boolean: bool = false; // boolean: 1 == true, 0 == false
    let character: char = 'a'; // char

    // tuples
    let tup: (i32, bool, char) = (1, true, 's'); // tuple: fixed length, immutable sequence. Cannot assign a tuple to a tuple of a different type (theoretically). Can use mut keyword to make it mutable.
    println!("{}", tup.2);
    let mut tup_two: (i32, bool, char) = (1, true, 's');
    tup_two.0 = 7;
    println!("{}", tup_two.0);

    // arrays
    let _array: [i32; 5] = [1, 2, 3, 4, 5]; // must have same type of element. Defined by type of element and # of elements. So, elements cannot be added.
    println!("{}", _array[2]);
    let mut _array_two = [1, 2, 3, 4, 5];
    _array_two[2] = 7;
    println!("{}", _array_two[2]);
    let _array_three: [i32; 5]; // does not automatically initialize values. (throws error on call)

    // prelude: list of things that Rust automatically imports into every program (basically stdlib). As small as possible. Focuses on things/traits common to almost every program.
    // crate == library == package
    // module is a specific piece of functionality within a crate
    // see use (like an import at the top) keyword
    // :: is the 'path separator operator' lets you go from pkg->module->fn/method

    // io
    let mut input = String::new(); // essentially a constructor for a String
    io::stdin().read_line(&mut input).expect("Failed to read line "); //typically when passing into functions a copy of the variable is created. &mut _var_ lets you modify the actual value of the variable in the outer scope from the fn/method
    // .expect catches any errors that may occur.
    // read_line() returns a Result object...
    println!("user input: {}", input);

    // cannot mix data types. Ie. i8 + u8 errors. Must convert types to sum even when obvious (i64 + i8).
    // overflows throw errors (sometimes).
    let z = 255.0f32;
    let zz = 255.0_f32;
    let zzz = 127_000i64; // == 127000
    let zzzz = 127_000 as i64; // explicit type conversion
    let yy = 10_i32;
    let zzzzz = zzzz / (yy as i64); // must manually/explicitly type/cast
    println!("{}", zzzzz);

    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Expected to read line");
    let int_input: i64 = input_2.trim().parse().unwrap();
    println!("input_2 (int): {}", int_input+2);

    // conditions
}
