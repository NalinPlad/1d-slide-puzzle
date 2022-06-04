use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::process;

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

    write!(stdout, r#"{}{}1,2,3 to swap keys. 
    (text from https://code.jsoftware.com/wiki/Scripts/1D_Rubik%27s_Cube)
    1D Rubik's Cube is a line of 6 numbers with
    original position:
   
      1 2 3 4 5 6
   
    which can be rotated in 3 different ways
    in groups of four:
        _______                _______
       (1 2 3 4)5 6  --(0)->  (4 3 2 1)5 6
          _______                _______
        1(2 3 4 5)6  --(1)->   1(5 4 3 2)6
            _______                _______
        1 2(3 4 5 6) --(2)->   1 2(6 5 4 3)
    
    
    "#, termion::cursor::Goto(1, 1), termion::clear::All)
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
            Key::Ctrl('c') => process::exit(0),
            _ => (),
        }
        for key in cube {
            print!(".{key}.");
        }
        stdout.flush().unwrap();
    }
        


    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();

    // match input.trim() {
    //     "1" => cube = swap(&cube,0),
    //     "2" => cube = swap(&cube,1),
    //     "3" => cube = swap(&cube,2),
    //     _ => println!("Invalid Input")
    // }
    // for key in cube {
    //     print!("| {key} |");
    // }
    // stdout().flush().unwrap();
    

}
