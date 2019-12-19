use crate::field::Field;

pub fn print_field(field: &Field) {
    println!("Field:");
    for line in 0..field.size_x() {
        for elem in 0..field.size_y() {
            if field.area()[line][elem].visible() == false {
                print!(" *");
            } else if field.area()[line][elem].mine() == true {
                print!(" M");
            } else if field.area()[line][elem].value() == 0 {
                print!(" _");
            } else {
                print!(" {}", field.area()[line][elem].value());
            }
        }
        println!();
    }
}
