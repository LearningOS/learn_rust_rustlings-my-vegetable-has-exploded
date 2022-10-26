// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
	Echo(String),
	Move(String),
	ChangeColor(String),
}

fn main() {
    println!("{:?}", Message::Quit(String::from("quit")));
    println!("{:?}", Message::Echo(String::from("echo")));
    println!("{:?}", Message::Move(String::from("move")));
    println!("{:?}", Message::ChangeColor(String::from("changecolor")));
}
