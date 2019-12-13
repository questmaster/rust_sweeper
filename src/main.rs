extern crate rand;

use rand::Rng;

const X_SIZE: usize = 5;
const Y_SIZE: usize = 5;

struct Field {
    area: [[u8; Y_SIZE]; X_SIZE],
}

fn print_field(field: &Field) {
    println!("Field:");
    for line in 0..X_SIZE {
        for elem in 0..Y_SIZE {
            print!(" {}", field.area[line][elem]);
        }
        println!();
    }
}

fn fill_mines_in_field(field: &mut Field, pct: u8)
{
    let mut rng = rand::thread_rng();
    for line in 0..X_SIZE {
        for elem in 0..Y_SIZE {
            if rng.gen_range(0, 100) < pct
            {
                field.area[line][elem] = 8;
                // TODO add counter method around bomb
            }
        }
    }
}

fn update_field(field: &mut Field) {
    // TODO
    field.area[2][2] = 2;
}

fn main() {
    let mut field: Field = Field {
        area: [[0_u8; Y_SIZE]; X_SIZE],
    };

    // TODO add game logic

    fill_mines_in_field(&mut field, 20);

    update_field(&mut field);

    print_field(&field);
}
