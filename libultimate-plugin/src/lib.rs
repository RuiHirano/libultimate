use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use skyline::nro::{self, NroInfo};
use skyline::nn::hid::{NpadGcState, GetNpadStyleSet};
mod charge;
use std::fs;
use std::fs::{OpenOptions};
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::sync::Mutex;
mod gamestate;
mod controlstate;
mod command;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

static GAMESTATE: Lazy<Mutex<gamestate::GameState>> = Lazy::new(|| Mutex::new(gamestate::GameState::default()));
static CONTROLSTATE: Lazy<Mutex<controlstate::ControlState>> = Lazy::new(|| Mutex::new(controlstate::ControlState::default()));
static COMMAND: Lazy<Mutex<command::Command>> = Lazy::new(|| Mutex::new(command::Command::default()));
static mut FIGHTER_MANAGER_ADDR: usize = 0;

pub fn handle_get_npad_state(state: *mut NpadGcState, _controller_id: *const u32){
    unsafe {
        match get_npad_state(state, _controller_id){
            Ok(_) => {},
            Err(_) => {},
        };
    }
}

unsafe fn get_npad_state(state: *mut NpadGcState, _controller_id: *const u32) -> Result<(), Error>{
    let player_id = *_controller_id + 1;
    let mut prev_control_state = CONTROLSTATE.lock().unwrap();
    let control_state = match controlstate::ControlState::get(player_id){
        Ok(cs) => cs,
        Err(_) => prev_control_state.clone(),
    };
    if control_state.player_id == player_id{
        //(*state).updateCount = control_state.update_count;
        if control_state.id != prev_control_state.id || control_state.hold {
            (*state).Buttons = control_state.buttons;
            (*state).LStickX = control_state.l_stick_x;
            (*state).LStickY = control_state.l_stick_y;
            (*state).RStickX = control_state.r_stick_x;
            (*state).RStickY = control_state.r_stick_y;
            (*state).Flags = control_state.flags;
            (*state).LTrigger = control_state.l_trigger;
            (*state).RTrigger = control_state.r_trigger;
            // update prev_control_state
            prev_control_state.id = control_state.clone().id;
            prev_control_state.player_id = control_state.clone().player_id;
            prev_control_state.update_count = control_state.clone().update_count;
            prev_control_state.buttons = control_state.clone().buttons;
            prev_control_state.l_stick_x = control_state.clone().l_stick_x;
            prev_control_state.l_stick_y = control_state.clone().l_stick_y;
            prev_control_state.r_stick_x = control_state.clone().r_stick_x;
            prev_control_state.r_stick_y = control_state.clone().r_stick_y;
            prev_control_state.flags = control_state.clone().flags;
            prev_control_state.l_trigger = control_state.clone().l_trigger;
            prev_control_state.r_trigger = control_state.clone().r_trigger;
            prev_control_state.hold = control_state.clone().hold;
        }
    }
    return Ok(());
}

#[skyline::hook(replace = ControlModule::get_command_flag_cat)]
pub unsafe fn handle_get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) -> i32 {
    // once per frame
    let mut flag = original!()(module_accessor, category);
    if category == FIGHTER_PAD_COMMAND_CATEGORY1 {
        save_gamestate(module_accessor);
        flag = match get_command_flag(module_accessor){
            Ok(flag) => flag,
            Err(_) => original!()(module_accessor, category),
        };
    }
    return flag;
}

unsafe fn get_command_flag(module_accessor: &mut app::BattleObjectModuleAccessor) -> Result<i32, Error>{
    let entry_id_int = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let player_id = entry_id_int + 1;
    let _command = command::Command::get(player_id)?;

    let mut prev_command = COMMAND.lock().unwrap();

    if player_id == _command.player_id {
        // execute stick until recieve next command
        ControlModule::set_main_stick_x(module_accessor, _command.stick_x);
        ControlModule::set_main_stick_y(module_accessor, _command.stick_y);

        // execute command once per recieve command
        if _command.id != prev_command.id {
            // update prev_command
            prev_command.id = _command.clone().id;
            prev_command.player_id = _command.clone().player_id;
            prev_command.action = _command.clone().action;
            prev_command.stick_x = _command.clone().stick_x;
            prev_command.stick_y = _command.clone().stick_y;
            // action
            match _command.action {
                command::Action::AIR_ESCAPE => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE);
                }
                command::Action::ATTACK_HI3 => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3);
                }
                command::Action::ATTACK_HI4 => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4);
                }
                command::Action::ATTACK_LW3 => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3);
                }
                command::Action::ATTACK_LW4 => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4);
                }
                command::Action::ATTACK_N => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);
                }
                command::Action::ATTACK_S3 => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3);
                }
                command::Action::ATTACK_S4 => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4);
                }
                command::Action::CATCH => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_CATCH);
                }
                command::Action::DASH => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_DASH);
                }
                command::Action::ESCAPE => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE);
                }
                command::Action::ESCAPE_B => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B);
                }
                command::Action::ESCAPE_F => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F);
                }
                command::Action::JUMP => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_JUMP);
                }
                command::Action::JUMP_BUTTON => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON);
                }
                command::Action::SPECIAL_ANY => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY);
                }
                command::Action::SPECIAL_HI => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI);
                }
                command::Action::SPECIAL_LW => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW);
                }
                command::Action::SPECIAL_N => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N);
                }
                command::Action::SPECIAL_S => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S);
                }
                command::Action::TURN => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_TURN);
                }
                command::Action::TURN_DASH => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH);
                }
                command::Action::WALK => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_WALK);
                }
                command::Action::WALL_JUMP_LEFT => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT);
                }
                command::Action::WALL_JUMP_RIGHT => {
                    return Ok(*FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT);
                }
                _ => return Err(Error::new(ErrorKind::Other, "NO CMD")),
            }
            //prev_command.update(_command);
        }
    }
    
    return Err(Error::new(ErrorKind::Other, "NO CMD"));
}

unsafe fn save_gamestate(module_accessor: &mut app::BattleObjectModuleAccessor){
    let entry_id_int = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
    let entry_id = app::FighterEntryID(entry_id_int);
    let x = PostureModule::pos_x(module_accessor);
    let y = PostureModule::pos_y(module_accessor);
    let lr = PostureModule::lr(module_accessor); //left or right
    let button_attack = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK);
    let button_special = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
    let button_smash = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SMASH);
    let button_guard = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD);
    let button_guard_hold = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD);
    let button_catch = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH);
    let button_jump = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP);
    let button_jump_mini = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI);
    let button_invalid = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_INVALID);
    let stick_x = ControlModule::get_stick_x(module_accessor);
    let stick_y = ControlModule::get_stick_y(module_accessor);
    let percent = DamageModule::damage(module_accessor, 0);
    let situation_kind = StatusModule::situation_kind(module_accessor);
    let fighter_kind = app::utility::get_kind(module_accessor);
    let fighter_status_kind = StatusModule::status_kind(module_accessor);
    let is_dead = StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD;
    //let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    //let fighter_info = app::FighterManager::get_fighter_information(fighter_manager, entry_id);
    let _charge = charge::get_charge(module_accessor, fighter_kind);
    let is_cpu = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == gamestate::FighterId::CPU as i32;

    let player_state = gamestate::PlayerState {
        id: entry_id_int,
        fighter_kind: fighter_kind,
        fighter_status_kind: fighter_status_kind,
        situation_kind: situation_kind,
        lr: lr,
        percent: percent,
        position: gamestate::Position{
            x: x,
            y: y,
        },
        control_state: gamestate::ControlState{
            stick_x: stick_x,
            stick_y: stick_y,
            button_attack: button_attack,
            button_special: button_special,
            button_smash: button_smash,
            button_guard: button_guard,
            button_guard_hold: button_guard_hold,
            button_catch: button_catch,
            button_jump: button_jump,
            button_jump_mini: button_jump_mini,
            button_invalid: button_invalid,
        },
        is_cpu: is_cpu,
        is_dead: is_dead,
        //charge: _charge,
    };
    let mut game_state = GAMESTATE.lock().unwrap();
    let mut exist = false;
    for (i, ps) in game_state.players.iter().enumerate() {
        if ps.id == player_state.id {
            game_state.players[i] = player_state;
            exist = true;
            break;
        }
    }
    if !exist {
        game_state.players.push(player_state);
    }
    gamestate::GameState::save(&game_state);
}

#[allow(improper_ctypes)]
extern "C" {
    fn add_nn_hid_hook(callback: fn(*mut NpadGcState, *const u32));
}

fn nro_main(nro: &NroInfo<'_>) {
    if nro.module.isLoaded {
        return;
    }

    if nro.name == "common" {
        unsafe {
            if (add_nn_hid_hook as *const ()).is_null() {
                panic!("The NN-HID hook plugin could not be found and is required to add NRO hooks. Make sure libnn_hid_hook.nro is installed.");
            }
            add_nn_hid_hook(handle_get_npad_state);
        }
        skyline::install_hooks!(
            handle_get_command_flag_cat,
        );
    }
}

fn touch(path: &Path) -> Result<(), Error> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn create_data() {
    println!("[libultimate] create data.");
    fs::create_dir_all("sd:/libultimate").expect("could not create data directory.");
    touch(&Path::new("sd:/libultimate/game_state.json")).expect("Error on creating game_state.json.");
    touch(&Path::new("sd:/libultimate/config.json")).expect("Error on creating config.json.");
    touch(&Path::new("sd:/libultimate/command.json")).expect("Error on creating command.json.");
}

#[skyline::main(name = "libultimate-plugin")]
pub fn main() {
    println!("[libultimate] Initializing...");
    //globalGameState.set(gamestate::GameState::default());
    create_data();
    nro::add_hook(nro_main).unwrap();
    println!("[libultimate] Finished Initializing.");
}
