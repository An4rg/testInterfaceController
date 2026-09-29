use raylib::prelude::*;

pub struct StringDisplayComponent {
    text: String,
    padding: i32,
    color: Color,
    positionX: i32,
    positionY: i32,
    sizeX: i32,
    sizeY: i32,
}

impl StringDisplayComponent {
    fn getText(&self) -> &str {
        &self.text
    }

    fn setText(&mut self, text: String) {
        self.text = text;
    }

    fn getPadding(&self) -> i32 {
        self.padding
    }

    fn setPadding(&mut self, padding: i32) {
        self.padding = padding;
    }

    fn getColor(&self) -> Color {
        self.color
    }

    fn setColor(&mut self, color: Color) {
        self.color = color;
    }

    pub fn create(
        text: String,
        padding: i32,
        color: Color,
        positionX: i32,
        positionY: i32,
        sizeX: i32,
        sizeY: i32,
    ) -> StringDisplayComponent {
        StringDisplayComponent {
            text,
            padding,
            color,
            positionX,
            positionY,
            sizeX,
            sizeY,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rounded(
            Rectangle::new(
                self.positionX as f32,
                self.positionY as f32,
                self.sizeX as f32,
                self.sizeY as f32,
            ),
            0.1,
            10,
            self.color,
        );
    }
}
