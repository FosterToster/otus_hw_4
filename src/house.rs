use crate::errors::SmartHomeError;
use crate::storage::SmartHomeStorage;
use std::collections::HashMap;

pub struct House<T: SmartHomeStorage> {
    name: String,
    storage: T,
    rooms: HashMap<String, Vec<String>>,
}

impl<T: SmartHomeStorage> House<T> {
    pub fn new(name: String, storage: T) -> Self {
        Self {
            name,
            storage,
            rooms: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_assigned_rooms(&self) -> Vec<String> {
        self.rooms.keys().cloned().collect::<Vec<String>>()
    }

    pub fn get_room_assigned_devices(
        &self,
        room_name: &str,
    ) -> Result<Vec<String>, SmartHomeError> {
        match self.rooms.get(room_name) {
            Some(devices) => Ok(devices.clone()),
            None => Result::Err(SmartHomeError {
                text: format!("This house does not contain a room named '{}'", room_name),
            }),
        }
    }

    pub fn add_room(&mut self, room_name: &str) -> Result<(), SmartHomeError> {
        //ensure unique
        if self.rooms.contains_key(room_name) {
            return Result::Err(SmartHomeError {
                text: format!("This house alredy contains a room named '{}'", room_name),
            });
        }

        //try commit
        match self.storage.add_room(&self.name, room_name) {
            Ok(_) => {
                self.rooms.insert(room_name.to_string(), Vec::new());
                Ok(())
            }
            Err(_) => Err(SmartHomeError {
                text: "Storage returned an error while adding a room".to_string(),
            }),
        }
    }

    pub fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<(), SmartHomeError> {
        //ensure unique
        match self.rooms.get(room_name) {
            Some(devices) => {
                if devices.contains(&device_name.to_string()) {
                    return Result::Err(SmartHomeError {
                        text: format!(
                            "The room '{}' of this house alredy contains device named '{}'",
                            room_name, device_name
                        ),
                    });
                }
            }
            None => {
                return Result::Err(SmartHomeError {
                    text: format!("This house does not contain a room named '{}'", room_name),
                })
            }
        };

        //try commit
        match self.storage.add_device(&self.name, room_name, device_name) {
            Ok(_) => {
                self.rooms
                    .get_mut(room_name)
                    .unwrap()
                    .push(device_name.to_string());
                Ok(())
            }
            Err(_) => Result::Err(SmartHomeError {
                text: "Storage returned an error while adding a device".to_string(),
            }),
        }
    }

    pub fn get_report(&self) -> Result<String, SmartHomeError> {
        let mut report = format!("Full report for house '{}'\n\r", self.name());

        for (room_name, devices) in self.rooms.iter() {
            report.push_str(&format!("\troom '{}'\r\n", room_name));

            for device_name in devices {
                if let Ok(device_status) =
                    self.storage
                        .get_device_status(&self.name, room_name, device_name)
                {
                    report.push_str(&format!("\t\tdevice '{}': ", device_name));
                    report.push_str(&device_status);
                    report.push_str("\n\r");
                } else {
                    return Result::Err( SmartHomeError{text: format!("Storage returned an error while getting status of device '{}' in room '{}'", device_name, room_name)} );
                }
            }
        }

        Ok(report)
    }
}
