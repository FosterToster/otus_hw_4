use homework_3::{House, SmartHomeStorage, StorageError};

// Define your own storage for devices in your house
struct MyStorage {}

// and implement SmartHomeStorage trait for it
impl SmartHomeStorage for MyStorage {
    fn add_device(
        &mut self,
        _house: &str,
        _room_name: &str,
        _device_name: &str,
    ) -> Result<(), StorageError> {
        Ok(())
    }

    fn remove_device(
        &mut self,
        _house: &str,
        _room_name: &str,
        _device_name: &str,
    ) -> Result<(), StorageError> {
        Ok(())
    }

    fn add_room(&mut self, _house: &str, _room_name: &str) -> Result<(), StorageError> {
        Ok(())
    }

    fn remove_room(&mut self, _house: &str, _room_name: &str) -> Result<(), StorageError> {
        Ok(())
    }

    fn get_device_status(
        &self,
        house: &str,
        room_name: &str,
        device_name: &str,
    ) -> Result<String, StorageError> {
        Ok(format!(
            "Device '{}' in room '{}' in house '{}' works fine",
            device_name, room_name, house
        ))
    }
}

fn main() {
    // create an instance of Home with your storage
    let mut my_home = House::new(String::from("My home"), MyStorage {});

    // now you are able to manage your home rooms and its assigned devices
    my_home.add_room("kitchen").unwrap();
    // but be careful, rooms are unique inside of house.
    // my_home.add_room("kitchen").unwrap(); // <- this will panic

    // let`s add some devices to kitchen
    my_home.add_device("kitchen", "light bulb").unwrap();
    my_home.add_device("kitchen", "speaker").unwrap();
    my_home.add_device("kitchen", "camera").unwrap();
    // remember that devices are also unique, but inside of rooms
    // my_home.add_device("kitchen", "camera").unwrap(); // <- this will panic

    // let`s practice
    my_home.add_room("living room").unwrap();
    my_home.add_device("living room", "socket").unwrap();
    my_home.add_device("living room", "blinds").unwrap();
    my_home.add_device("living room", "thermometer").unwrap();

    // now you can get list your rooms
    println!("rooms:");
    my_home
        .get_assigned_rooms()
        .iter()
        .for_each(|room| println!("\t{}", room));

    // and devices of any defined room
    println!("kitchen devices:");
    my_home
        .get_room_assigned_devices("kitchen")
        .unwrap()
        .iter()
        .for_each(|device| println!("\t{}", device));

    // you can also get a complete report on the status of devices in your home
    println!("{}", my_home.get_report().unwrap());
}
