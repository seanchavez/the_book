fn main() {
    fn main() {
        let reference_to_nothing = no_dangle();
    }

    fn no_dangle() -> String {
        let s = String::from("hello");

        s
    }

    // fn dangle() -> &String {
    //     let s = String::from("hello");

    //     &s
    // }

    // let  mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    //println!("{}, {}", r1, r2);

   // change(&mut s1);

    // let len = calculate_length(&s1);

    // println!("The length of {} is {}", s1, len);
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
