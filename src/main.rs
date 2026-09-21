fn main() {
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);

    /*
    Different data types

    let _my_num = 5; // integer
    let _my_double = 5.99; // float
    let _my_letter = 'D'; // character
    let _my_bool = true; // boolean
    let _my_text = "Hello"; // string
     */

    // Arithmetic op
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);

    if logged_in {
        println!("Welcome back!");
    } else {
        println!("Please log in.");
    }

    // Using if as an Expression
    let time = 18;
    let greetings = if time < 18 {
        "Good Day"
    } else {
        "Good Evening"
    };
    println!("{greetings}!");

    // Rust does not have a ternary operator (cond ? val1 : val2)

    let day = 4;
    match day {
        // kind of like switch
        1 => println!("Mon"),
        2 => println!("Tue"),
        3 => println!("Wed"),
        4 => println!("Thu"),
        5 => println!("Fri"),
        6 => println!("Sat"),
        7 => println!("Sun"),
        _ => println!("Invalid day!"),
    }

    let day = 6;
    // multiple match
    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    // match with a return value
    let result = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    };

    println!("{}", result);

    // loops in rust, it also returns value using break
    let mut loop_count = 1;
    let result = loop {
        println!("This will run 3 times: {loop_count}");
        if loop_count == 3 {
            break loop_count;
        }
        loop_count += 1;
    };

    println!("Result: {result}");

    // other types of loop includes while and for
    // while loop will run as long as the condition is true
    let mut while_loop_count = 1;
    while while_loop_count < 4 {
        println!("While loop count: {while_loop_count}");
        while_loop_count += 1;
    }
    // while also stops with break

    // for loop in rust don't need to handle counter var
    // here 1..6 means from 1 up to (but not including) 6
    for i in 1..6 {
        println!("i is: {i}");
    }

    // if you want to have inclusive range, use ..=
    for i in 1..=6 {
        println!("i is {i}");
    }

    // Just like other loops, you can use break to stop the loop and continue to skip a value

    // functions in rust
    fn add_two(a: i32, b: i32) -> i32 {
        // return a + b;
        a + b // you can omit the return keyword and the semicolon
        // The last line a + b is automatically returned
    }

    let res = add_two(3, 7);
    println!("{res}");

    // Strings in rust
    let greetings = "Hello";
    let text1 = "World".to_string();
    let text2 = String::from("Hello World");

    let result_string = format!("{} & {} & {}", greetings, text1, text2);
    println!("{result_string}");

    // modifying string in rust
    let mut word = String::from("Hi");
    word.push('!'); // You can add one char using push
    println!("{word}"); // Hi!

    // concatenate strings
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let result_s1_s2 = format!("{}, {}!", s1, s2);
    println!("{result_s1_s2}");

    let new_result = &result_s1_s2;
    println!("{new_result}");

    // You can use the .len() method to get the length of a string
    println!("Length of concatenated string: {}", result_s1_s2.len());
}
