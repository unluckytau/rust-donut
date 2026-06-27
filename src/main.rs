use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main(){
    // terminal size used in the original
    const WIDTH: usize = 80;
    const HEIGHT: usize = 22;
    const BUF: usize = WIDTH * HEIGHT;
    
    // rotation angles
    // let mut a: f32 = 0.0;
    // let mut b: f32 = 0.0;

    // chars as luminance
    // let palette = ".,-~:;=!*#$@".as_bytes();

    loop{
        let screen: [char; BUF] = [' '; BUF];
        print!("\x1b[H");

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = x + y * WIDTH;
                print!("{}", screen[index]);
            }
            println!();
        }
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(30));
        
        // still trying to understand the math vro T_T
    }
}
