use crate::field::Field;

pub fn print_field(field: &Field) {
    println!("Field:");
    // todo not dynamic
    println!("      0  1  2  3  4  5  6  7  8  9");
    println!("   +------------------------------");
    for line in 0..field.size_x() {
        print!(" {} |", line);
        for elem in 0..field.size_y() {
            if field.area()[line][elem].visible() == false {
                print!("  *");
            } else if field.area()[line][elem].mine() == true {
                print!("  M");
            } else if field.area()[line][elem].value() == 0 {
                print!("  _");
            } else {
                print!("  {}", field.area()[line][elem].value());
            }
        }
        println!();
    }
}
