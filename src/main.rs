use std::env;

pub mod d_10;
pub mod d_11;
pub mod d_12;
pub mod d_13;
pub mod d_14;
pub mod d_15;
pub mod d_16;
pub mod d_17;
pub mod d_18;
pub mod d_19;
pub mod d_20;
pub mod d_21;
pub mod d_22;
pub mod d_23;
pub mod d_3;
pub mod d_4;
pub mod d_5;
pub mod d_6;
pub mod d_8;
pub mod d_9;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "3" => d_3::main(),
        "4" => d_4::main(),
        "5" => d_5::main(),
        "6" => d_6::main(),
        "8" => d_8::main(),
        "9" => d_9::main(),
        "10" => d_10::main(),
        "11" => d_11::main(),
        "12" => d_12::main(),
        "13" => d_13::main(),
        "14" => d_14::main(),
        "15" => d_15::main(),
        "16" => d_16::main(),
        "17" => d_17::main(),
        "18" => d_18::main(),
        "19" => d_19::main(),
        "20" => d_20::main(),
        "21" => d_21::main(),
        "22" => d_22::main(),
        "23" => d_23::main(),
        _ => println!("I don't know what to say!"),
    }
}
