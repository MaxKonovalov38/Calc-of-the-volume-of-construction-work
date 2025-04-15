use std::io;
use std::io::Write;
use std::process;

mod trench;

/// Крутизна откосов в зависимости от вида грунта и глубины выемки
fn get_the_coefficient() -> f64 {
    let coefficient: f64;

    print!("Введите высоту траншеи: ");
    io::stdout().flush().unwrap();
    let mut str_num_two = String::new();
    io::stdin().read_line(&mut str_num_two).expect("ERROR");
    let number_two: f64 = str_num_two.trim().parse().unwrap();

    println!(
        "Наименование грунтов:
    1-- Насыпной неуплотненный;
    2-- Песчаный и гравийный;
    3-- Cупесь;
    4-- Суглинок;
    5-- Глина;
    6-- Лессы и лессовидные"
    );

    print!("Выберите класс грунтов: ");
    io::stdout().flush().unwrap();
    let mut str_num_three = String::new();
    io::stdin().read_line(&mut str_num_three).expect("ERROR");
    let number_three: u32 = str_num_three.trim().parse().unwrap();

    if number_three == 1 {
        if number_two <= 1.50 {
            coefficient = 0.67
        } else if number_two > 1.50 && number_two <= 3.00 {
            coefficient = 1.00
        } else {
            coefficient = 1.25
        }
    } else if number_three == 2 {
        if number_two <= 1.50 {
            coefficient = 0.05
        } else {
            coefficient = 1.00
        }
    } else if number_three == 3 {
        if number_two <= 1.50 {
            coefficient = 0.25
        } else if number_two > 1.50 && number_two <= 3.00 {
            coefficient = 0.67
        } else {
            coefficient = 0.85
        }
    } else if number_three == 4 {
        if number_two <= 1.50 {
            coefficient = 0.00
        } else if number_two > 1.50 && number_two <= 3.00 {
            coefficient = 0.50
        } else {
            coefficient = 0.75
        }
    } else if number_three == 5 {
        if number_two <= 1.50 {
            coefficient = 0.00
        } else if number_two > 1.50 && number_two <= 3.00 {
            coefficient = 0.25
        } else {
            coefficient = 0.50
        }
    } else if number_three == 6 {
        if number_two <= 1.50 {
            coefficient = 0.00
        } else {
            coefficient = 0.50
        }
    } else {
        eprintln!("[**ERROR**] Введенный вами параметр не верный");
        process::exit(0x0100);
    }

    coefficient
}

fn main() {
    println!(
        "Калькулятор расчета объемов земляных работ:
    1 -- Траншея с вертикальными стенками на спланированной местности
    2 -- Траншея с вертикальными стенками, с перепадом высот
    3 -- Траншея с откосами на спланированной местности
    
    0 -- Выход из программы."
    );

    print!("Enter: ");
    io::stdout().flush().unwrap();
    let mut str_num_one = String::new();
    io::stdin().read_line(&mut str_num_one).expect("ERROR");
    let number_one: u32 = str_num_one.trim().parse().unwrap();

    match number_one {
        0 => process::exit(0x0100),
        1 => trench::trench_vwalls_plannes_terrain(1 as f64, 2 as f64, 6 as f64),
        2 => trench::trench_vwalls_height_difference(1 as f64, 2 as f64, 3 as f64, 6 as f64),
        3 => {
            println!("hello: {}", get_the_coefficient());
        }
        _ => {
            eprintln!("[**ERROR**] Введенный вами параметр не верный");
            process::exit(0x0100);
        }
    }
}
