use std::io;
use std::io::{Write};
use std::process;

mod trench;

fn main() {
    println!("Калькулятор расчета объемов земляных работ:
    1 -- Траншея с вертикальными стенками на спланированной местности
    2 -- Траншея с вертикальными стенками, с перепадом высот
    3 -- Траншея с откосами на спланированной местности
    
    0 -- Выход из программы.");

    print!("Enter: ");
    io::stdout().flush().unwrap();
    let mut str_num_one = String::new();
    io::stdin().read_line(&mut str_num_one)
        .expect("Error!!!");
    let number_one: i32 = str_num_one.trim().parse().unwrap();

    match number_one {
        0 => process::exit(0x0100),
        1 => trench::trench_vwalls_plannes_terrain(1 as f64,2 as f64,6 as f64),
        2 => trench::trench_vwalls_height_difference(1 as f64,2 as f64,3 as f64,6 as f64),
        _ => println!("erroer!")
    }
}