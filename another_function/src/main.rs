fn main() {
    println!("Hello, world!");
    another_function(5, 'm');
}

fn another_function(value: i32, unit_label: char) {
    println!("The mesurement is: {}{}", value, unit_label);
}