use raylib::prelude::*;

struct StringDisplayComponent {
    text: String,
    padding: i32,
    color: Color,
    positionX: i32,
    positionY: i32,
    sizeX: i32,
    sizeY: i32,
}

impl StringDisplayComponent {
    fn getText(&self ) -> String {
        return self.text;
    }

    fn setText(&mut self, text: String) {
        self.color = text;
    }
    
    fn getPadding(&self) -> i32 {
        return self.padding;
    }

    fn setPadding(&mut self, padding: i32) {
        self.padding = padding;
    }

    fn getColor(&self) -> i32 {
        return self.color;
    }

    fn setColor(&mut self, color: i32) {
        self.color = color;
    }
}

impl StringDisplayComponent {
    fn create(text: String, padding: i32, color: Color, positionX: i32,positionY: i32, sizeX: i32, sizeY: i32)
    {
        return StringDisplayComponent {
            text: text, padding: padding, color: color,
            positionX: positionX, positionY: positionY,
            sizeX: sizeX, sizeY; sizeY
        }
    }
}
