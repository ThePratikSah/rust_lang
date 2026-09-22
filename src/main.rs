use std::collections::HashMap;

fn main() {
    // let mut x = 5;
    // println!("Before: {}", x);
    // x = 10;
    // println!("After: {}", x);

    // /*
    // Different data types

    // let _my_num = 5; // integer
    // let _my_double = 5.99; // float
    // let _my_letter = 'D'; // character
    // let _my_bool = true; // boolean
    // let _my_text = "Hello"; // string
    //  */
    // // Arithmetic op
    // let add = 5 + 3;
    // let sub = 10 - 4;
    // let mul = 6 * 2;
    // let div = 12 / 3;
    // let rem = 10 % 3;

    // println!("Add: {}", add);
    // println!("Sub: {}", sub);
    // println!("Mul: {}", mul);
    // println!("Div: {}", div);
    // println!("Rem: {}", rem);

    // let logged_in = true;
    // let is_admin = false;

    // println!("Is regular user: {}", logged_in && !is_admin);
    // println!("Has any access: {}", logged_in || is_admin);
    // println!("Not logged in: {}", !logged_in);

    // if logged_in {
    //     println!("Welcome back!");
    // } else {
    //     println!("Please log in.");
    // }

    // // Using if as an Expression
    // let time = 18;
    // let greetings = if time < 18 {
    //     "Good Day"
    // } else {
    //     "Good Evening"
    // };
    // println!("{greetings}!");

    // // Rust does not have a ternary operator (cond ? val1 : val2)

    // let day = 4;
    // match day {
    //     // kind of like switch
    //     1 => println!("Mon"),
    //     2 => println!("Tue"),
    //     3 => println!("Wed"),
    //     4 => println!("Thu"),
    //     5 => println!("Fri"),
    //     6 => println!("Sat"),
    //     7 => println!("Sun"),
    //     _ => println!("Invalid day!"),
    // }

    // let day = 6;
    // // multiple match
    // match day {
    //     1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    //     6 | 7 => println!("Weekend"),
    //     _ => println!("Invalid day"),
    // }

    // // match with a return value
    // let result = match day {
    //     1 => "Monday",
    //     2 => "Tuesday",
    //     3 => "Wednesday",
    //     4 => "Thursday",
    //     5 => "Friday",
    //     6 => "Saturday",
    //     7 => "Sunday",
    //     _ => "Invalid day.",
    // };

    // println!("{}", result);

    // // loops in rust, it also returns value using break
    // let mut loop_count = 1;
    // let result = loop {
    //     println!("This will run 3 times: {loop_count}");
    //     if loop_count == 3 {
    //         break loop_count;
    //     }
    //     loop_count += 1;
    // };

    // println!("Result: {result}");

    // // other types of loop includes while and for
    // // while loop will run as long as the condition is true
    // let mut while_loop_count = 1;
    // while while_loop_count < 4 {
    //     println!("While loop count: {while_loop_count}");
    //     while_loop_count += 1;
    // }
    // // while also stops with break

    // // for loop in rust don't need to handle counter var
    // // here 1..6 means from 1 up to (but not including) 6
    // for i in 1..6 {
    //     println!("i is: {i}");
    // }

    // // if you want to have inclusive range, use ..=
    // for i in 1..=6 {
    //     println!("i is {i}");
    // }

    // // Just like other loops, you can use break to stop the loop and continue to skip a value

    // // functions in rust
    // fn add_two(a: i32, b: i32) -> i32 {
    //     // return a + b;
    //     a + b // you can omit the return keyword and the semicolon
    //     // The last line a + b is automatically returned
    // }

    // let res = add_two(3, 7);
    // println!("{res}");

    // // Strings in rust
    // let greetings = "Hello";
    // let text1 = "World".to_string();
    // let text2 = String::from("Hello World");

    // let result_string = format!("{} & {} & {}", greetings, text1, text2);
    // println!("{result_string}");

    // // modifying string in rust
    // let mut word = String::from("Hi");
    // word.push('!'); // You can add one char using push
    // println!("{word}"); // Hi!

    // // concatenate strings
    // let s1 = String::from("Hello");
    // let s2 = String::from("World");
    // let result_s1_s2 = format!("{}, {}!", s1, s2);
    // println!("{result_s1_s2}");

    // let new_result = &result_s1_s2; // borrowed ownership
    // println!("{new_result}");

    // let result_clone = result_s1_s2.clone(); // if you actually want to create a copy
    // println!("{result_clone}"); // Explicit deep copy of heap data of 'result_s1_s2'

    // // You can use the .len() method to get the length of a string
    // println!("Length of concatenated string: {}", result_s1_s2.len());

    let mut user = User {
        name: String::from("Pratik"),
        age: 32,
    };

    user.greet_user();
    user.get_age();
    user.birthday();
    user.get_age();

    // let mut name = user.name;

    // println!("{}", name);
    // println!("{}", user.age);
    // println!("{}", user.name); when accessing user.name,
    // this was throwing error because the name ownership was moved to name var

    // let mut car_name = String::from("Honda");
    // let new_car_name = &mut car_name;

    // new_car_name.push_str(" Amaze");

    // println!("{new_car_name}");
    // println!("{}", car_name);

    // let japnese_char: char = 'ひ';
    // println!("{japnese_char}");

    // greet_user(&mut name);
    // println!("After the function call - {name}");

    // let borrowed_name = &mut name;
    // greet_user(borrowed_name);

    // println!("Finally {borrowed_name}");
    // println!("And Name - {name}");

    // let arr: [u32; 3] = [1, 3, 5]; // size is fixed, can be modified
    // println!("{}", arr[0]);

    // let mut vector_array: Vec<u32> = vec![1, 3, 5];

    // vector_array.push(4);

    // let random_number = vector_array[3];
    // println!("{random_number}");
    // println!("{}", vector_array[3]);

    // let mut user_names = vec![String::from("Pratik"), String::from("Sah")];
    // user_names.push(String::from("Tanya"));

    // println!("{}", user_names[1]);
    // let title_of_user = &user_names[1]; // you can't move out a string from a vector, just borrow
    // println!("{title_of_user}");

    // // tuples lets you hold multiple values of different types
    // let person: (String, i32, bool) = (String::from("John"), 30, true);
    // println!("Name: {}", person.0);
    // println!("Age: {}", person.1);
    // println!("Is active: {}", person.2);

    // Tuples are often used to return multiple values from function
    // fn get_user() -> (String, i32) {
    //     (String::from("Liam"), 25)
    // }

    // let (person_name, person_age, person_is_active) = &person;
    // println!("P - {person_name} {person_age} {person_is_active}");

    // let person_name = person.0;
    // println!("{person_name}");

    // // println!("{}", person.0); this will not work as the value was moved to person_name

    let mut map = HashMap::new();
    map.insert("pratik", "Sah"); // if you insert a new value using existing key, it will replace the old one

    // let title = map.get("pratik"); // you can get the data like this from the map
    // println!("{:?}", title); // but this will wrap the value with Some(value)

    if let Some(val) = map.get("pratik") {
        println!("The key exists: {val}");
    } else {
        println!("Key not found");
    }

    map.remove("pratik"); // remove the key from map

    // loop through the HashMap
    for (key, val) in map {
        // since we are directly moving the ownership of map here, we won't be able to use map later after the loop
        println!("Key: {key}, Val: {val}");
    }

    // map.insert("1", "2"); // this will throw error

    // let fruits = vec!["apple", "banana", "orange"];
    // for fruit in &fruits {
    //     // and not just fruits as it will move the ownership from fruits
    //     // to the for loop and it will not be accessible after the loop
    //     println!("I like {}.", fruit);
    // }
    // println!("{:?}", fruits);
}

struct User {
    name: String, // heap data
    age: i32,     // stack data
}

impl User {
    fn greet_user(&self) {
        println!("{}", self.name);
    }

    fn get_age(&self) {
        println!("{}", self.age);
    }

    fn birthday(&mut self) {
        // mutable self so that we can update the value
        self.age += 1;
    }
}

// fn greet_user(name: &mut String) {
//     println!("Hello {name}");
//     name.push_str(" Sah");
// }
