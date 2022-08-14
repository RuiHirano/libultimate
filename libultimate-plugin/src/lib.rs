use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterBase;
use smash::app::sv_system;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use skyline::nro::{self, NroInfo};
mod charge;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io;
use std::path::Path;
mod gamestate;
use once_cell::sync::OnceCell;

static globalGameState: OnceCell<gamestate::GameState> = OnceCell::new();

#[skyline::hook(replace = ControlModule::get_command_flag_cat)]
pub unsafe fn handle_get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) -> i32 {
    // once per frame
    if category == FIGHTER_PAD_COMMAND_CATEGORY1 {
        let x = PostureModule::pos_x(module_accessor);
        let y = PostureModule::pos_y(module_accessor);
        let lr = PostureModule::lr(module_accessor); //left or right
        let guard = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD);
        let catch = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH);
        let jump = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP);
        let percent = DamageModule::damage(module_accessor, 0);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let fighter_kind = app::utility::get_kind(module_accessor);
        let fighter_status_kind = StatusModule::status_kind(module_accessor);
        let fighter_status_catch = StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH;
        let _charge = charge::get_charge(module_accessor, fighter_kind);
        let player_state = gamestate::PlayerState {
            fighter_kind: fighter_kind,
            situation_kind: situation_kind,
            lr: lr,
            percent: percent,
            position: gamestate::Position{
                x: x,
                y: y,
            },
            //charge: _charge,
        };
        let game_state = gamestate::GameState {
            players: Box::new([player_state]),
            projectiles: Box::new([]),
        };
        gamestate::save_game_state(game_state);
        println!("[libultimate] fighter change status. category: {}, x {}, y {}, lr {}, guard {}, jump {}, catch {}, catch2 {},", category, x, y, lr, guard, jump, catch, fighter_status_catch);
    }
    return original!()(module_accessor, category);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterBase_change_status)]
pub unsafe fn handle_change_status(
    _fighter: &mut L2CFighterBase,
    _status_kind: L2CValue,
    _unk: L2CValue,
){
    let module_accessor = sv_system::battle_object_module_accessor(_fighter.lua_state_agent);
    let fighter_kind = app::utility::get_kind(module_accessor);
    let status_kind_int = _status_kind
        .try_get_int()
        .unwrap_or(*FIGHTER_STATUS_KIND_WAIT as u64) as i32;

    let attack = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK);
    let stick_x = ControlModule::get_stick_x(module_accessor);
    let stick_y = ControlModule::get_stick_y(module_accessor);
    let x = PostureModule::pos_x(module_accessor);
    let y = PostureModule::pos_y(module_accessor);
    let lr = PostureModule::lr(module_accessor); //left or right
    let percent = DamageModule::damage(module_accessor, 0);
    let situation_kind = StatusModule::situation_kind(module_accessor);
    let _charge = charge::get_charge(module_accessor, fighter_kind);
    //println!("[libultimate] fighter change status. {}, status {}, percent {}, xy {} {}, lr {}, attack {}", fighter_kind, status_kind_int, percent, x, y, lr, attack);
    //let stick_x = ControlModule::set_main_stick_x(module_accessor, 1.0);
    let player_state = gamestate::PlayerState {
        fighter_kind: fighter_kind,
        situation_kind: status_kind_int,
        lr: lr,
        percent: percent,
        position: gamestate::Position{
            x: x,
            y: y,
        },
        //charge: _charge,
    };
    let game_state = gamestate::GameState {
        players: Box::new([player_state]),
        projectiles: Box::new([]),
    };
    gamestate::save_game_state(game_state);
    original!()(_fighter, _status_kind, _unk);
}

fn nro_main(nro: &NroInfo<'_>) {
    println!("[libultimate] nro module.");
    if nro.module.isLoaded {
        return;
    }
    println!("[libultimate] nro module.2");

    if nro.name == "common" {
        skyline::install_hooks!(
            //handle_change_status,
            handle_get_command_flag_cat,
        );
    }
}

fn touch(path: &Path) -> io::Result<()> {
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
    globalGameState.set(gamestate::GameState::default());
    create_data();
    nro::add_hook(nro_main).unwrap();
    println!("[libultimate] Finished Initializing.");
}
