extern crate colored;
extern crate rgb;
extern crate ansi_rgb;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::process;
use rand::distributions::{Distribution, Uniform};
use ansi_rgb::{ Foreground, Background };
use rgb::RGB8;


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
    let mut moves: i64 = 0;

    // Palette
    let red = RGB8::new(255, 102, 99);
    let orange = RGB8::new(254, 177, 68);
    let yellow = RGB8::new(253, 253, 151);
    let green = RGB8::new(158, 224, 158);
    let blue = RGB8::new(158, 193, 207);
    let violet = RGB8::new(204, 153, 201);
    let black = RGB8::new(0, 0, 0);
    
    // 123564 -> 1 2
    // 561234 -> 

    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, r#"{}{}{}1,2,3 to swap keys. CTRL-S to (s)cramble. CTRL-C to ex(c)it. Press ENTER to start"#,
     termion::cursor::Hide,
      termion::cursor::Goto(1, 1),
       termion::clear::All)
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
            Key::Char('1') => {cube = swap(&cube,0); moves += 1;},
            Key::Char('2') => {cube = swap(&cube,1); moves += 1;},
            Key::Char('3') => {cube = swap(&cube,2); moves += 1;},

            Key::Ctrl('c') => {
                write!(stdout, r#"{}"#, termion::cursor::Show).unwrap();
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

                moves = 0;
            },
            _ => (),
        }
        if moves < 10 {
            print!("MOVES: {} ", moves.to_string().fg(green));
        } else if moves < 25 {
            print!("MOVES: {} ", moves.to_string().fg(yellow));
        } else {
            print!("MOVES: {} ", moves.to_string().fg(red));
        }


        for key in cube {
            match key { //rycbmg
                1 => print!("{}{}{}","  ".to_string().bg(red) ,key.to_string().bg(red).fg(black), "  ".to_string().bg(red)),
                2 => print!("{}{}{}","  ".to_string().bg(orange) ,key.to_string().bg(orange).fg(black), "  ".to_string().bg(orange)),
                3 => print!("{}{}{}","  ".to_string().bg(yellow) ,key.to_string().bg(yellow).fg(black), "  ".to_string().bg(yellow)),
                4 => print!("{}{}{}","  ".to_string().bg(green) ,key.to_string().bg(green).fg(black), "  ".to_string().bg(green)),
                5 => print!("{}{}{}","  ".to_string().bg(blue) ,key.to_string().bg(blue).fg(black), "  ".to_string().bg(blue)),
                6 => print!("{}{}{}","  ".to_string().bg(violet) ,key.to_string().bg(violet).fg(black), "  ".to_string().bg(violet)),
                _ => ()
            }
        }
        stdout.flush().unwrap();
}

}
