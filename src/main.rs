// https://rust-book.cs.brown.edu/ch05-03-method-syntax.html
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50
    };
    dbg!(&rect1);
    println!("Area: {}", rect1.area());

    let mut rect2 = Rect {
        width: 10,
        height: 40,
    };
    rect2.width = 11;
    let sq = Rect::square(60);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold sq? {}", rect1.can_hold(&sq));
}
