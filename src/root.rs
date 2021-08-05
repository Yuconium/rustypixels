use macroquad::prelude::*;
pub mod rect;


const TILESIZE: f32 = 25.0;

pub struct Main {
    Canvas: rect::Container,
    Options: rect::Container,
    color: Color
}


impl Main {
    pub fn new() -> Main{
        Main{
            Canvas: rect::Container::new(150.0, 0.0, 500.0, 500.0,
                                            BLUE,
                                            rect::Rectangle_function::Container),
            Options: rect::Container::new(0.0, 500.0, 800.0, 100.0,
                                            GREEN,
                                            rect::Rectangle_function::Container),
            color: BLACK
        }
    }


    pub fn generate_tiles(&mut self){
        let mut xpos: f32 = 0.0;
        let mut ypos: f32 = 0.0;
        let field_size = 500.0 / TILESIZE;
        for y in 0..field_size as usize {
            self.Canvas.contains.push(Vec::new());
            for x in 0..field_size as usize {
                self.Canvas.contains[y]
                    .push(rect::Rectangle::new(self.Canvas.rect.xpos + xpos,
                                                self.Canvas.rect.ypos + ypos,
                                                TILESIZE,
                                                TILESIZE,
                                                BLUE,
                                                rect::Rectangle_function::Tile));
                xpos += TILESIZE;
            }
            xpos = 0.0;
            ypos += TILESIZE;

        }
    }

    pub fn one_frame(&mut self){
        clear_background(WHITE);
        self.Canvas.draw(&mut self.color);
        self.Options.draw(&mut self.color);
        draw_text(&mouse_position().0.to_string(), 50.0, 50.0, 50.0, BLACK);
        draw_text(&mouse_position().1.to_string(), 50.0, 100.0, 50.0, BLACK);


    }
}
