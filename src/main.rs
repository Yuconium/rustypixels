use macroquad::prelude::*;
mod root;




#[macroquad::main("BasicShapes")]
async fn main() {
    let mut root: root::Main = root::Main::new();
    root.generate_tiles();
    println!("{}", screen_height());

    loop {
        root.one_frame();
        next_frame().await
    }
}
