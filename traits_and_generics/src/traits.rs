struct Canvas;

impl Canvas {
    fn write_at(x:i32, y:i32, c:char) ->() {

    }
}
trait Visible {
    fn draw(&self, canvas: &mut Canvas);

    fn hit_test(&self, x:i32, y:i32) -> bool;
}

struct Broom {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

/// To implement a trait, use the syntax impl TraitName for Type:
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}


/// If we wanted to add a helper method in support of Broom::draw(), we would have to define it in a
/// separate block
impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height -1 ..self.y
    }
}


pub struct Sink;

use std::io::{Write, Result};

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    //Apart from the above methods, the Write trait contains a lot of default methods, which are
    //not required to be implemented here.
}
