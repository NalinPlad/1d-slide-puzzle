extern crate colored;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::process;
use rand::distributions::{Distribution, Uniform};
use colored::*;


fn swap(inp: &[i8; 6], mv: i8) -> [i8; 6] {    
    match mv {
        // def = [0,1,2,3,4,5]
        0 => {
            let fl: [i8; 6] = [inp[3],inp[2],inp[1],inp[0],inp[4],inp[5]];
            return fl;
        },

        1 => {
            let fl: [i8; 6] = [inp[0],inp[4],inp[3],inp[2],inp[1],inp[5]];
            return fl;
        },

        2 => {
            let fl: [i8; 6] = [inp[0],inp[1],inp[5],inp[4],inp[3],inp[2]];
            return fl;
        }

        i8::MIN..=-1_i8 | 3_i8..=i8::MAX => todo!()
    }
}



fn main() {
    let mut cube: [i8; 6] = [1,2,3,4,5,6];

    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, r#"{}{}{}1,2,3 to swap keys. CTRL-S to (s)cramble. CTRL-C to exit. Press enter to start"#, termion::cursor::Hide, termion::cursor::Goto(1, 1), termion::clear::All)
            .unwrap();
    stdout.flush().unwrap();


    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('1') => cube = swap(&cube,0),
            Key::Char('2') => cube = swap(&cube,1),
            Key::Char('3') => cube = swap(&cube,2),

            Key::Ctrl('c') => {
                write!(stdout, r#"{}"#, termion::cursor::Show);
                process::exit(0);
            }, // Exit
            Key::Ctrl('s') => {
                let between = Uniform::from(0 .. 3);
                let mut rng = rand::thread_rng();

                // Scramble
                for _c in 1 .. 10000{
                    let throw = between.sample(&mut rng);
                    
                    match throw {
                        0 => cube = swap(&cube,0),
                        1 => cube = swap(&cube,1),
                        2 => cube = swap(&cube,2),
                        i32::MIN..=-1_i32 | 3_i32..=i32::MAX => todo!()
                    }
                }
            },
            _ => (),
        }
        for key in cube {
            match key {
                1 => print!("{}{}{}"," ".to_string().on_red() ,key.to_string().on_red(), " ".to_string().on_red()),
                2 => print!("{}{}{}"," ".to_string().on_green() ,key.to_string().on_green(), " ".to_string().on_green()),
                3 => print!("{}{}{}"," ".to_string().on_yellow() ,key.to_string().on_yellow(), " ".to_string().on_yellow()),
                4 => print!("{}{}{}"," ".to_string().on_cyan() ,key.to_string().on_cyan(), " ".to_string().on_cyan()),
                5 => print!("{}{}{}"," ".to_string().on_blue() ,key.to_string().on_blue(), " ".to_string().on_blue()),
                6 => print!("{}{}{}"," ".to_string().on_magenta() ,key.to_string().on_magenta(), " ".to_string().on_magenta()),
                i8::MIN..=0_i8 | 6_i8..=i8::MAX => todo!()
            }
        }
        stdout.flush().unwrap();
    }

}
