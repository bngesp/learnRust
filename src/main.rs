struct Figure;

impl Figure {

    fn new(&self){
        self.new()
    }
    fn surface(&self) -> i32{
        0
    }
}

struct Point {x: f64, y: f64}

impl Point {

    fn default() -> Point {
        Point{ x: 0.0, y: 0.0}
    }

    fn new(x: f64, y: f64) -> Point {
        Point{ x, y }
    }

}

struct Rectangle {p1: Point, p2: Point}

impl Rectangle {
    fn surface(&self) -> f64{
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);
impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}
fn main() {

    let rectangle = Rectangle {
        // Associated functions are called using double colons
        p1: Point::default(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle perimeter: {}", Rectangle::perimeter(&rectangle));
    println!("Rectangle area: {}", rectangle.surface());
}