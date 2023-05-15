use colored::*;

pub enum Colors {
    Red,
    Green,
    Yellow
}

pub fn cprint(string:&str, color:Colors)
{
    match color
    {
        Colors::Red => println!("{}",string.red()),
        Colors::Green => println!("{}",string.green()),
        Colors::Yellow => println!("{}",string.yellow()),
    };
}