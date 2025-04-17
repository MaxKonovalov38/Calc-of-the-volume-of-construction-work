/// Траншея с вертикальными стенками на спланированной местности
pub fn trench_vwalls_plannes_terrain(vec_numbers: Vec<f64>) {
    // vec_numbers[0] -- ширина траншеи
    // vec_numbers[1] -- высота траншеи
    // vec_numbers[2] -- длина траншеи
    // Формула расчета объема траншеи
    let volume_trench: f64 = vec_numbers[0] * vec_numbers[1] * vec_numbers[2];
    // Формула расчета площади поперечного сечения
    let cross_sectional_area: f64 = vec_numbers[0] * vec_numbers[1];

    println!(
        "Объем траншеи (м3): {}; Площадь поперечного сечения траншеи (м2): {}",
        volume_trench, cross_sectional_area
    );
}

/// Траншея с вертикальными стенками, с перепадом высот
pub fn trench_vwalls_height_difference(vec_numbers: Vec<f64>) {
    // vec_numbers[0] -- ширина траншеи
    // vec_numbers[1] -- h1 высота траншеи
    // vec_numbers[2] -- h2 высота траншеи
    // vec_numbers[3] -- длина траншеи
    // Формула расчета объема траншеи
    let volume_trench: f64 =
        vec_numbers[0] * ((vec_numbers[1] + vec_numbers[2]) / 2.0) * vec_numbers[3];
    // Формула расчета площади поперечного сечения F1
    let cross_sectional_area_one: f64 = vec_numbers[0] * vec_numbers[1];
    // Формула расчета площади поперечного сечения F2
    let cross_sectional_area_two: f64 = vec_numbers[0] * vec_numbers[2];

    println!(
        "Объем траншеи (м3): {}; Площадь поперечного сечения траншеи F1 (м2): {}; Площадь поперечного сечения траншеи F2 (м2): {}",
        volume_trench, cross_sectional_area_one, cross_sectional_area_two
    );
}

// Траншея с откосами на спланированной местности
pub fn trench_slopes_planned_area(param_tuple: ((f64, f64), f64, f64)) {
    let (c, h) = param_tuple.0;
    // Расчет размера по ширине верха траншеи
    // Формула расчета объема траншеи
    //let volume_trench: f64 = ((width_trench_base + width_trench_top) / 2.00) * height_trench * length_trench;
    // Формула расчета площади поперечного сечения
    //let cross_sectional_area: f64 = ((width_trench_base + width_trench_top) / 2.00) * height_trench;

    // Расчет по коэффициенту
    // Формула расчета ширины верха траншеи
    let width_trench_top: f64 = h * c + param_tuple.1 + h * c;
    // Формула расчета объема траншеи
    let volume_trench: f64 = ((param_tuple.1 + width_trench_top) / 2.00) * h * param_tuple.2;
    // Формула расчета площади поперечного сечения
    let cross_sectional_area: f64 = ((param_tuple.1 + width_trench_top) / 2.00) * h;
    println!(
        "Ширина верха траншеи (м): {}; Объем траншеи (м3): {}; Площадь поперечного сечения траншеи (м2): {}",
        width_trench_top, volume_trench, cross_sectional_area
    );
}
