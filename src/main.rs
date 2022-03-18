// `F` must be gen


// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: FnOnce()>(f: F) {
    f();
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    let sum_of_squared_odd_numbers: u32 =
        (0..).
    println!("functional style: {}", sum_of_squared_odd_numbers);
}