use std::{io, result};

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
    //io::stdin().read_line(&mut input).expect("Failed to read line "); //typically when passing into functions a copy of the variable is created. &mut _var_ lets you modify the actual value of the variable in the outer scope from the fn/method
    // .expect catches any errors that may occur.
    // read_line() returns a Result object...
    //println!("user input: {}", input);

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
    //io::stdin().read_line(&mut input_2).expect("Expected to read line");
    //let int_input: i64 = input_2.trim().parse().unwrap();
    //println!("input_2 (int): {}", int_input+2);

    // conditions
    // comparing objects of different types will cause problems; convert
    // operator precedence: !, &&, ||
    let f = "cookie";
    if f == "cookie" {
        println!("Too true");
    } // basically java syntax

    // functions
    // fn main in main.rs is program entry point
    // naming convention: snake_case
    // location in file does not matter (not like C)
    // Rust fn's can return an expression but not a statement. Refresher:
    // Expression: macro, fn call, const, literal, anything that evaluates to a value
    // Statement: variable declaration, doesn't evaluate to anything, ex: let statement
    // - you can have an expression inside of a statement. Ex:
    let number = {
        let x = 3;
        x + 1 // no semicolon means return this expression. No semi colon would cause error... 'didnt return anything'
    };
    println!("Expr in Stmt: {}", number);
    test();
    let result = test_params(8, 7);
    println!("result: {}", result);

    // memory mgmt
    // systems programming language - kinda want to know where info is stored
    // Stack: LIFO/FILO; fastest; only capable of storing fixed sized data; parameters w/out &mut are duplicated - gets most recent value from top of the stack
    // Heap: Non-traditional; dynamically sized; Ex: String::from(some_str) fn - dynamically sized object; implements memory allocator; stack stores name and ptr to value in heap

}

fn test() {
    println!("called 'test' fn");
}

fn test_params(x: i32, y: i32) -> i32 {
    println!("Sum: {}", x + y);
    x + y // returned cuz' no ;... can be an explicit return. Ex: return x + y; (Used for returning before the last line in a fn
}
