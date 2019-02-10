fn main() {
let s = String::from("hello");
takes_ownership(s);
//println!("Oh yeah {}?",s);

let x = 5;
makes_copy(x);
println!("Oh yeah {}?",x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}
