use macroquad::prelude::*;


pub enum Rectangle_function {
    Container,
    ColorChangeButton,
    Tile
}
pub struct Rectangle {
    pub xpos: f32,
    pub ypos: f32,
    width: f32,
    height: f32,
    color: Color ,
    pub contains: Vec<Vec<Rectangle>>,
    function_kind: Rectangle_function,


}

impl Rectangle {
    pub fn new(xpos: f32, ypos: f32, width: f32, height: f32, color: Color, function_kind: Rectangle_function) -> Rectangle {
        Rectangle {
            xpos,
            ypos,
            width,
            height,
            color,
            contains: Vec::new(),
            function_kind

        }
    }



    fn function(&mut self,  color: &mut Color) {
        match self.function_kind {
            Rectangle_function::Container => println!("HELLO"),
            Rectangle_function::ColorChangeButton=> *color = GREEN,
            Rectangle_function::Tile => self.color = *color,
        }
    }
    fn mouse_hover(&self) -> bool {
        if mouse_position().0 >= self.xpos && mouse_position().0 <= self.xpos + self.width
        && mouse_position().1  >= self.ypos && mouse_position().1 <= self.ypos + self.height {
            true
        }   else {
            false
        }
    }





    pub fn draw(&mut self, color: &mut Color){
        draw_rectangle(self.xpos, self.ypos, self.width, self.height, self.color);

        for y in self.contains.iter_mut() {
            for x in y {
                draw_rectangle(x.xpos, x.ypos, x.width, x.height, x.color);

                if x.mouse_hover() == true {
                    draw_line(x.xpos, x.ypos, x.xpos + x.width, x.ypos, 5.0, GREEN);
                    draw_line(x.xpos, x.ypos, x.xpos, x.ypos + x.height, 5.0, RED);
                    draw_line(x.xpos, x.ypos + x.height, x.xpos + x.width, x.ypos + x.height, 8.0, MAGENTA);
                    draw_line(x.xpos + x.width, x.ypos, x.xpos + x.width, x.ypos + x.height, 5.0, YELLOW);

                    if is_mouse_button_down(MouseButton::Left) {
                        x.function(color);
                    }   else if is_mouse_button_down(MouseButton::Right) {
                        x.color = BLUE;
                    }

                }
            }
        }
    }

}
