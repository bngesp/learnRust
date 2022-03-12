use std::intrinsics::sqrtf32;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,

}

fn rect_area(rect: Rectangle) -> f32{
    let Point { x: a, y: b } = rect.top_left;
    let Point { x: p_a, y: p_b } = rect.bottom_right;
    let longuer = a*a + p_a * p_a ;
    let larguer = b*b + p_b * p_b ;
    //unsafe { sqrtf32(longuer) }
    unsafe { sqrtf32(longuer) * sqrtf32(larguer) }
}