mod my_mod {
    fn test_closure() {
        fn function(i: i32) -> i32 { i + 1 }
        println!("la valeu de la fontion {}", function(32));

        let closure_annotated = |i: i32| -> i32 { i + 1 };
        let closure_inferred = |i| i + 1;
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

    fn test_main() {


        // `double` satisfies `apply_to_3`'s trait bound
        let double = |x| 2 * x;
        let double2 = || 2 * x;

        println!("3 doubled: {}", apply_to_3(double));
        println!("2 doubled: {}", doubleval(double));
    }

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

    pub fn closure_main() {
        println!("Find the sum of all the squared odd numbers under 1000");
        let upper = 1000;

        // Imperative approach
        // Declare accumulator variable
        let mut acc = 0;
        // Iterate: 0, 1, 2, ... to infinity
        for n in 0.. {
            // Square the number
            let n_squared = n * n;

            if n_squared >= upper {
                // Break loop if exceeded the upper limit
                break;
            } else if is_odd(n_squared) {
                // Accumulate value, if it's odd
                acc += n_squared;
            }
        }
        println!("imperative style: {}", acc);

        // Functional approach
        let sum_of_squared_odd_numbers: u32 =
            (0..).map(|n| n * n)                             // All natural numbers squared
                .take_while(|&n_squared| n_squared < upper) // Below upper limit
                .filter(|&n_squared| is_odd(n_squared))     // That are odd
                .fold(0, |acc, n_squared| acc + n_squared); // Sum them
        println!("functional style: {}", sum_of_squared_odd_numbers);
    }
}