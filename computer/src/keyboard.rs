use device_query::{DeviceQuery, DeviceState};
pub(crate) const KEY_COUNT: u8 = 96;

pub(crate) struct Keyboard
{

}

impl Keyboard
{
    pub(crate) fn new() -> Self
    {
        Self {}
    }

    pub(crate) fn get_keys(&self) -> Vec<u8>
    {
        let mut key_codes: Vec<u8> = Vec::new();
        let keys = DeviceState::new().get_keys();
        for key in keys
        {
            key_codes.push(key as u8);
        }
        return key_codes;
    }
}