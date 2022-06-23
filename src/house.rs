use crate::errors::SmartHomeError;
use crate::storage::SmartHomeStorage;
use std::collections::{HashMap, HashSet};

pub struct House<T: SmartHomeStorage> {
    name: String,
    storage: T,
    rooms: HashMap<String, HashSet<String>>,
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
            Some(devices) => Ok(devices.iter().cloned().collect::<Vec<String>>()),
            None => Err(SmartHomeError::NotFound(format!(
                "This house does not contain a room named '{}'",
                room_name
            ))),
        }
    }

    pub fn add_room(&mut self, room_name: &str) -> Result<(), SmartHomeError> {
        //ensure unique
        if self.rooms.contains_key(room_name) {
            return Result::Err(SmartHomeError::NotUnique(format!(
                "This house alredy contains a room named '{}'",
                room_name
            )));
        }

        //try commit
        self.storage
            .add_room(&String::from(self.name()), room_name)?;

        self.rooms.insert(room_name.into(), HashSet::new());

        Ok(())
    }

    pub fn remove_room(&mut self, room_name: &str) -> Result<(), SmartHomeError> {
        if !self.rooms.contains_key(room_name) {
            return Err(SmartHomeError::NotFound(format!(
                "This house does not contains a room named '{}'",
                room_name
            )));
        };

        let devices = self.rooms.get(room_name).unwrap();

        if !devices.is_empty() {
            return Err(SmartHomeError::NotEmpty(format!(
                "The room named {} is not empty",
                room_name
            )));
        }

        self.rooms.remove(room_name).unwrap();

        Ok(())
    }

    pub fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<(), SmartHomeError> {
        match self.rooms.get_mut(room_name) {
            Some(devices) => {
                if !devices.insert(device_name.into()) {
                    return Err(SmartHomeError::NotUnique(format!(
                        "The room '{}' of this house alredy contains device named '{}'",
                        room_name, device_name
                    )));
                }
            }
            None => {
                return Err(SmartHomeError::NotFound(format!(
                    "This house does not contain a room named '{}'",
                    room_name
                )))
            }
        }

        //try commit
        self.storage
            .add_device(&String::from(self.name()), room_name, device_name)?;

        Ok(())
    }

    pub fn remove_device(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<(), SmartHomeError> {
        let devices = match self.rooms.get_mut(room_name) {
            Some(devices) => devices,
            None => {
                return Err(SmartHomeError::NotFound(format!(
                    "This house does not contains a room named {}",
                    room_name
                )));
            }
        };

        if !devices.remove(device_name) {
            return Err(SmartHomeError::NotFound(format!(
                "The room named {} does not contain device named {}",
                room_name, device_name
            )));
        }

        Ok(())
    }

    pub fn get_report(&self) -> Result<String, SmartHomeError> {
        let mut report = format!("Full report for house '{}'\n\r", self.name());

        for (room_name, devices) in self.rooms.iter() {
            report.push_str(&format!("\troom '{}'\r\n", room_name));

            for device_name in devices {
                let device_status =
                    self.storage
                        .get_device_status(self.name(), room_name, device_name)?;
                report.push_str(&format!("\t\tdevice '{}': ", device_name));
                report.push_str(&device_status);
                report.push_str("\n\r");
            }
        }

        Ok(report)
    }
}
