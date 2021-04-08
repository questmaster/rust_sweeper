use crate::game_area::GameArea;

pub fn print_area(area: &GameArea) {
    println!("Game area:");
    // todo not dynamic
    println!("   X  0  1  2  3  4  5  6  7  8  9");
    println!(" Y +------------------------------");
    for elem in 0..area.size_y() {
        print!(" {} |", elem);
        for line in 0..area.size_x() {
            if area.area()[line][elem].visible == false {
                print!("  *");
            } else if area.area()[line][elem].mine == true {
                print!("  M");
            } else if area.area()[line][elem].value == 0 {
                print!("  _");
            } else {
                print!("  {}", area.area()[line][elem].value);
            }
        }
        println!();
    }
}
