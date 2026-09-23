fn main() {
    let s = String::from("My favorite game is League of Legends.");
    test_taking_ownership(s);
    
    println!("{s}");

    let current_target_id = 2669168;
    test_making_copy(x);
}

fn test_taking_ownership(some_string: String) {
    println!("{some_string}");
}

fn test_making_copy(some_integer: i32) {
    println!("{some_integer}");
}