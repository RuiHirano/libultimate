//use crate::charge::ChargeState;
use std::io::{Write, BufReader, Error, ErrorKind};
use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, File};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct ControlState {
    pub id: String,
    pub player_id: u32,
    pub update_count: i64,
    pub buttons: u64,
    pub l_stick_x: i32,
    pub l_stick_y: i32,
    pub r_stick_x: i32,
    pub r_stick_y: i32,
    pub flags: u32,
    pub l_trigger: u32,
    pub r_trigger: u32,
    pub hold: bool,
}

impl Default for ControlState {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            player_id: 0,
            update_count: 0,
            buttons: 0,
            l_stick_x: 0,
            l_stick_y: 0,
            r_stick_x: 0,
            r_stick_y: 0,
            flags: 0,
            l_trigger: 0,
            r_trigger: 0,
            hold: false,
        }
    }
}

impl ControlState {
    pub fn get(entry_id: u32) -> Result<ControlState, Error>{
        let mut control_state: ControlState = ControlState::default();
        let control_state_ok_path = format!("sd:/libultimate/control_state_{}.ok.json", entry_id);
        let control_state_path = format!("sd:/libultimate/control_state_{}.json", entry_id);
        if Path::new(&control_state_ok_path).exists() {
            let file = File::open(&control_state_path)?;
            let reader = BufReader::new(file);
            control_state = serde_json::from_reader(reader)?;
            // remove ok.json
            fs::remove_file(&control_state_ok_path).unwrap();
        }else{
            return Err(Error::new(ErrorKind::Other, "control_state.json does not exist"));
        }
        return Ok(control_state);
    }
}
