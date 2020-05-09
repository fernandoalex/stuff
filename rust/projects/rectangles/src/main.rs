#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height 
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {  width: size, height: size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
           width: 5,
           height: 1, 
        };

        assert!(larger.can_hold(&smaller));
    }
}

fn main() {
    let rect1 = Rectangle {  width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("rect1 is {:?}", rect1);


    let rect2 = Rectangle {  width: 10, height: 40 };
    let rect3 = Rectangle {  width: 60, height: 45 };

    println!("can react1 hold react2? {}", rect1.can_hold(&rect2));
    println!("can react1 hold react3? {}", rect1.can_hold(&rect3));

    // this does not work
    //let square1 = Rectangle.square(3);

    let square1 = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        square1.area()
    );
}

