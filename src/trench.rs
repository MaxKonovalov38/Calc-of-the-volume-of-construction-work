/// Траншея с вертикальными стенками на спланированной местности
pub fn trench_vwalls_plannes_terrain (width_trench: f64, height_trench: f64, length_trench: f64) {
    // Формула расчета объема траншеи
    let volume_trench: f64 = width_trench * height_trench * length_trench;
    // Формула расчета площади поперечного сечения
    let cross_sectional_area: f64 = width_trench * height_trench;

    println!("Объем траншеи (м3): {}; Площадь поперечного сечения траншеи (м2): {}", volume_trench, cross_sectional_area);
}

/// Траншея с вертикальными стенками, с перепадом высот
pub fn trench_vwalls_height_difference (width_trench: f64, height_trench_one: f64, height_trench_two: f64, length_trench: f64) {
    // Формула расчета объема траншеи
    let volume_trench: f64 = width_trench * ((height_trench_one + height_trench_two) / 2.0) * length_trench;
    // Формула расчета площади поперечного сечения F1
    let cross_sectional_area_one: f64 = width_trench * height_trench_one;
    // Формула расчета площади поперечного сечения F2
    let cross_sectional_area_two: f64 = width_trench * height_trench_two;

    println!("Объем траншеи (м3): {}; Площадь поперечного сечения траншеи F1 (м2): {}; Площадь поперечного сечения траншеи F2 (м2): {}",
        volume_trench, cross_sectional_area_one, cross_sectional_area_two);
}

// Траншея с откосами на спланированной местности
//pub fn trench_slopes_planned_area () {}