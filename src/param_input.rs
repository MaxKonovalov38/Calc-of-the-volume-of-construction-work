use std::io;
use std::io::Write;
use std::process;

/// Параметры ввода пользователя (3 параметра)
pub fn parametrs_three() -> Vec<f64> {
    print!(
        "Ввод параметров через ПРОБЕЛ, по окончанию нажмите ENTER;\nВведите ширину, высоту, длину траншеи (м): "
    );
    io::stdout().flush().unwrap();

    let mut reader: String = String::new();
    io::stdin().read_line(&mut reader).ok().expect("read error");

    let numbers: Vec<f64> = reader
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    // Проверка количества параметров
    if numbers.len() < 3 || numbers.len() > 3 {
        eprintln!(
            "[** ERROR **] -- The number of parameters you entered does not match the required number"
        );
        process::exit(0x0100);
    }

    numbers
}

/// Параметры ввода пользователя (4 параметра)
pub fn parametrs_four() -> Vec<f64> {
    print!(
        "Ввод параметров через ПРОБЕЛ, по окончанию нажмите ENTER;\nВведите ширину, высоту h1, высоту h2, длину траншеи (м): "
    );
    io::stdout().flush().unwrap();

    let mut reader: String = String::new();
    io::stdin().read_line(&mut reader).ok().expect("read error");

    let numbers: Vec<f64> = reader
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    // Проверка количества параметров
    if numbers.len() < 4 || numbers.len() > 4 {
        eprintln!(
            "[** ERROR **] -- The number of parameters you entered does not match the required number"
        );
        process::exit(0x0100);
    }

    numbers
}