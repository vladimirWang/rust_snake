extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BLACK_COLOR:Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [
            to_coord_u32(width), to_coord_u32(height),
        ]
    ).exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BLACK_COLOR, g);
            game.draw(&c, g);
        });
        // window.draw_2d(&event, |c, g| {  
        //     clear(BLACK_COLOR, g);  

        //     // 使用 graphics 的函数和方法来绘制形状  
        //     rectangle(  
        //         [0.80, 0.00, 0.00, 1.0], // 红色  
        //         [0.0, 0.0, 100.0, 100.0], // 位置和大小  
        //         c.transform,  
        //         g,  
        //     );  

        //     // 使用 SpriteBatch 绘制图像（如纹理）  
        //     // 这里假设你已经加载了一个纹理并将其放入 SpriteBatch 中  
        //     // batch.draw(&default_texture, &draw_state, c.transform, g).unwrap();  

        //     // 其他绘制代码...  
        // });  
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
