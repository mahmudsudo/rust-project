use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // println!("Hello, world!");
    // panic!("crash and burn");
    let f = File::open("head.txt");
    let f = match f { Ok(file) => file, 
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
             match File::create("hello.txt") {
                  Ok(fc) => fc, 
                  Err(e) => {
                       panic!( "Tried to create file but there was a problem: {:?}", e ) }, 
                    } },
                     Err(error) =>
                      { panic!( "There was a problem opening the file: ");}
};
}