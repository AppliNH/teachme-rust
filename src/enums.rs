enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // match is an equivalent of switch
    match m {
        Movement::Up => println!("Up!"),
        Movement::Down => println!("Down!"),
        Movement::Left => println!("Left!"),
        Movement::Right => println!("Right!")
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let _avatar2 = Movement::Up;
    let _avatar3 = Movement::Right;
    let _avatar4 = Movement::Down;
    move_avatar(avatar1)
}