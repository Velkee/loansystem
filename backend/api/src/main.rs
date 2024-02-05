use self::models::*;
use diesel::prelude::*;
use loansystem_backend::*;

fn main() {
    use self::schema::devices::dsl::*;

    let connection = &mut establish_connection();
    let results = devices
        .select(Device::as_select())
        .load(connection)
        .expect("Error loading devices");

    for device in results {
        println!("{}", device.id);
        println!("----------");

        if let Some(serial) = device.serial_number {
            println!("{}", serial);
            println!("----------");
        }

        if let Some(cat) = device.category {
            println!("{}", cat);
            println!("----------");
        }

        println!("{}", device.status);
    }
}
