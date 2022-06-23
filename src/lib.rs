mod errors;
mod house;
mod storage;

pub use errors::*;
pub use house::House;
pub use storage::SmartHomeStorage;

#[cfg(test)]
mod tests {
    use crate::House;
    use crate::SmartHomeStorage;
    use crate::StorageError;
    struct DummyStorage {}

    impl SmartHomeStorage for DummyStorage {
        fn add_room(&mut self, _house: &str, _room_name: &str) -> Result<(), StorageError> {
            Ok(())
        }

        fn remove_room(&mut self, _house: &str, _room_name: &str) -> Result<(), StorageError> {
            Ok(())
        }

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

        fn get_device_status(
            &self,
            _house: &str,
            _room_name: &str,
            _device_name: &str,
        ) -> Result<String, StorageError> {
            Ok(String::from("Works fine"))
        }
    }

    #[test]
    fn add_room() {
        let mut house = House::new("My home".to_string(), DummyStorage {});
        house.add_room("Main").unwrap();
    }

    #[test]
    fn remove_room() {
        let mut house = House::new("My home".to_string(), DummyStorage {});
        house.add_room("Main").unwrap();
        assert!(!house.get_assigned_rooms().is_empty());
        house.remove_room("Main").unwrap();
        assert!(house.get_assigned_rooms().is_empty());
    }

    #[test]
    #[should_panic]
    fn add_room_unique() {
        let mut house = House::new("My home".to_string(), DummyStorage {});

        house.add_room("Main").unwrap();

        //should panic
        house.add_room("Main").unwrap();
    }

    #[test]
    fn add_device() {
        let mut house = House::new("My home".to_string(), DummyStorage {});
        house.add_room("Main").unwrap();
        house.add_device("Main", "Thermometer").unwrap();
    }

    #[test]
    #[should_panic]
    fn remove_room_not_empty() {
        let mut house = House::new("My home".to_string(), DummyStorage {});
        house.add_room("Main").unwrap();
        house.add_device("Main", "Socket").unwrap();
        assert!(!house.get_assigned_rooms().is_empty());
        house.remove_room("Main").unwrap();
    }

    #[test]
    fn remove_device() {
        let mut house = House::new("My home".to_string(), DummyStorage {});
        house.add_room("Main").unwrap();
        house.add_device("Main", "Socket").unwrap();
        house.remove_device("Main", "Socket").unwrap();
        assert!(house.get_room_assigned_devices("Main").unwrap().is_empty());
    }

    #[test]
    #[should_panic]
    fn remove_device_not_found() {
        let mut house = House::new("My home".to_string(), DummyStorage {});
        house.add_room("Main").unwrap();
        house.add_device("Main", "Socket").unwrap();
        house.remove_device("Main", "NotSocket").unwrap();
    }

    #[test]
    #[should_panic]
    fn add_device_unique() {
        let mut house = House::new("My home".to_string(), DummyStorage {});
        house.add_room("Main").unwrap();

        house.add_device("Main", "Thermometer").unwrap();

        //should panic
        house.add_device("Main", "Thermometer").unwrap();
    }

    #[test]
    fn assigned_rooms() {
        let mut rooms = vec!["First".to_string(), "Second".to_string()];
        let mut house = House::new("My home".to_string(), DummyStorage {});

        for room_name in &rooms {
            house.add_room(room_name).unwrap();
        }

        rooms.sort();
        let mut rooms_assigned = house.get_assigned_rooms();
        rooms_assigned.sort();

        assert_eq!(rooms_assigned, rooms);
    }

    #[test]
    fn assigned_devices() {
        let mut devices = vec!["First".to_string(), "Second".to_string()];
        let room_name = String::from("Main");
        let mut house = House::new("My home".to_string(), DummyStorage {});

        house.add_room(&room_name).unwrap();

        for device_name in &devices {
            house.add_device(&room_name, device_name).unwrap();
        }

        if let Ok(mut room_devices) = house.get_room_assigned_devices(&room_name) {
            room_devices.sort();
            devices.sort();
            assert_eq!(devices, room_devices)
        } else {
            panic!("room was not addet")
        }
    }

    #[test]
    fn full_report() {
        let devices = vec!["First".to_string(), "Second".to_string()];
        let rooms = vec!["First".to_string(), "Second".to_string()];
        let mut house = House::new("My home".to_string(), DummyStorage {});

        for room_name in rooms {
            if house.add_room(&room_name).is_err() {
                panic!("Can`t add room")
            }

            for device_name in &devices {
                if house.add_device(&room_name, device_name).is_err() {
                    panic!("Can`t add device")
                }
            }
        }

        if let Ok(report) = house.get_report() {
            println!("{}", report)
        } else {
            panic!("Full report error")
        }
    }
}
