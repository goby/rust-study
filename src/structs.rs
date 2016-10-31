use std;
use std::fmt;
use std::ops::{Add, Mul};
use std::fmt::{Debug,Display};

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Display for Point<T> where T: Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Add for Point<T> where T: Add<Output=T> + Copy {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {x: self.x + other.x, y: self.y + other.y }
    }
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

#[derive(Debug)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

struct Point3dRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
    z: &'a mut i32,
}

struct Coord(i32);

//////enum///////
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move{ x: i32, y: i32},
    Write(String)
}

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64
}


// associate type
trait Graph {
    type Node: Display;
    type Edge;

    fn has_edge(&self, &Self::Node, &Self::Node) -> bool;
    fn edges(&self, &Self::Node) -> Vec<Self::Edge>;
}

type Node = Point<i32>;

type Edge = Line<i32>;

struct TestGraph;

impl Graph for TestGraph {
    type Node = Node;
    type Edge = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    } 

    fn edges(&self, node: &Node) -> Vec<Edge> {
        Vec::new()
    }
}

trait HasArea<T> {
    fn area(&self) -> T;
}

struct Square<T> {
    centor: Point<T>,
    side: T,
}

impl<T> HasArea<T> for Square<T>
    where T: Mul<Output=T> + Copy {
    fn area(&self) -> T {
        self.side * self.side
    }    
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle{x: self.x, y: self.y, radius: self.radius + increment}
    }

}


#[test]
pub fn main() {
    let c = self::Circle{x: 12.0, y: 30.0, radius: 1.11};
    println!("c           = {:?}", c);
    println!("c.grow(0.5) = {:?}", c.grow(0.5));
    println!("c.area() == {}", c.area());
    println!("c.grow(0.5).area == {}", c.grow(0.5).area());

    let g   = TestGraph;
    let obj = Box::new(g) as Box<Graph<Node=Node, Edge = Edge>>;

    let p1 = Point{x: 1, y: 10};
    let p2 = Point{x: 100, y: 1111};
    let p = p1 + p2;
    assert_eq!(p.x, 101);
    assert_eq!(p.y, 1121);

    let sq = Square{ centor: p, side: 20};
    assert_eq!(sq.area(), 400);

    let sq = Square{ centor: Point{x: 0.0f64, y: 0.0f64}, side: 12.0f64};
    assert_eq!(sq.area(), 12.0f64 * 12.0f64);
}
