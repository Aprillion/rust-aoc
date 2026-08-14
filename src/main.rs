// https://rust-book.cs.brown.edu/ch05-02-example-structs.html
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
fn main() {
    let scale = 2;
    let rect1 = Rect {
        width: dbg!(30 * scale),
        height: 50
    };

    dbg!(&rect1);
    println!("Area: {}", area(&rect1));
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
