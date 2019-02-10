fn main() {
// let s = String::from("hello");
// takes_ownership(s);
// //println!("Oh yeah {}?",s);

// let x = 5;
// makes_copy(x);
// println!("Oh yeah {}?",x);
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer)

// let s1 = gives_ownership();

// let s2 = String::from("hello");

// let s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of {} is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}