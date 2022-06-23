use crate::errors::StorageError;

pub trait SmartHomeStorage {
    fn add_room(&mut self, house: &str, room_name: &str) -> Result<(), StorageError>;
    fn remove_room(&mut self, house: &str, room_name: &str) -> Result<(), StorageError>;
    fn add_device(
        &mut self,
        house: &str,
        room_name: &str,
        device_name: &str,
    ) -> Result<(), StorageError>;
    fn remove_device(
        &mut self,
        house: &str,
        room_name: &str,
        device_name: &str
    ) -> Result<(), StorageError>;
    fn get_device_status(
        &self,
        house: &str,
        room_name: &str,
        device_name: &str,
    ) -> Result<String, StorageError>;
}
