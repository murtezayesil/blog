//  add square.rs as a MODule
//  mod PATH_TO_FILE (without .rs extension) ;
mod square;

fn main() {
    let number = 4;
    println!("Square of {} is {}", number, square::square_of(number) );
}
