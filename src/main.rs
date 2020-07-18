/// Test Branch

fn main() {
    // print_numbers_to(10);
    println!("{}", is_even(2345));
    println!("Hello, World!");
}

// fn print_numbers_to(num: u32) -> () {
//     for n in 1..num {
//         println!("{}", n);
//     }
// }

fn is_even(num: u32) -> bool {
    num % 2 == 0
}
