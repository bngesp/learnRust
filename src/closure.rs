fn test_closure() {
    fn function(i: i32) -> i32 { i +1}
    println!("la valeu de la fontion {}", function(32));

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i| i + 1  ;
    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || "juste une valeur";
    println!("closure returning one: {}", one());
}
eric.
fn apply<F>(f: F) where F: FnOnce() {
    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
// The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn doubleval<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    let mut alpha = 0;
    for a in 1..10 {
        alpha = alpha + f(a);
    }
    alpha
}
fn main() {


    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;
    let double2 = || 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
    println!("2 doubled: {}", doubleval(double));
}