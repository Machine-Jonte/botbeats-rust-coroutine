#![allow(unused_must_use)]
use crate::modules::laser;
use crate::types::Position;
use itertools::Itertools;
use lazy_static::lazy_static;
use ordered_float;
use rbot::messages::RMsgComponentStatus;
use rbot::{self, component_state};
use std::ops::Coroutine;
use std::pin::Pin;
use std::sync::RwLock;

lazy_static! {
    // Stores the position of the enemy with the latest known sensor
    // information. The prediction for the enemy robot can contiously be updated
    // with better estimation.
    static ref ENEMY_POSITION: RwLock<Position> = RwLock::new(Position::default());
}

// Entry point for your code. The `lib.rs` file requires a main function here,
// serving as the interface for the game to interact with user-defined code.
// Feel free to create new files, modules, and use online packages. The only
// requirement is that your code must compile to WebAssembly.
//
// Documentation is available at: https://docs.rs/rbot/latest/rbot/
pub fn main() {
    let mut move_routine = #[coroutine]
    || {
        let mut last_move_time = rbot::time().unwrap();
        let mut move_sign = 1.0;

        loop {
            let current_time = rbot::time().unwrap();
            let elapsed_time = current_time - last_move_time;
            if elapsed_time > 2.5 {
                move_sign *= -1.0;
                last_move_time = current_time;
            }
            let position = ENEMY_POSITION.read().unwrap().clone();
            let angle = position.angle();
            let [x, y] = rbot::conversions::angle_to_xy(angle + 45.0 * move_sign);
            rbot::velocity(x, y, 1.0);
            yield;
        }
    };

    let mut repair_routine = #[coroutine]
    || loop {
        yield;
        let component_states = (0..4)
            .map(|comp_id| component_state(comp_id).unwrap())
            .filter(|state| state.health > 0.0)
            .collect_vec();

        let with_least_hp = component_states
            .iter()
            .min_by_key(|comp_state| ordered_float::OrderedFloat(comp_state.health));

        if let Some(with_least_hp) = with_least_hp {
            let comp_id = with_least_hp.component_id;
            if let Ok(hp) = rbot::modules::repair(comp_id).and_then(|m| Ok(m.healed_amount)) {
                rbot::print(&format!("Healed component with ID: {comp_id}, {hp}HP \n"));
            }
        }
    };

    let mut aim_routine = #[coroutine]
    || loop {
        yield;
        let component_states = (0..4)
            .map(|comp_id| component_state(comp_id).unwrap())
            .filter(|state| state.health > 0.0)
            .collect_vec();

        let comp_id: Option<i32> = {
            let activated_component: Option<&RMsgComponentStatus> = component_states
                .iter()
                .filter(|comp_state| comp_state.is_activated)
                .next();

            // If there is a component that is activated, use that one. Else,
            // use the component with the smallest cooldown.
            if let Some(comp_state) = activated_component {
                Some(comp_state.component_id)
            } else {
                component_states
                    .iter()
                    .min_by_key(|comp_state| ordered_float::OrderedFloat(comp_state.cooldown))
                    .and_then(|v| Some(v.component_id))
            }
        };

        if comp_id.is_none() {
            rbot::print("Warning, no suitable component found.\n");
            break;
        }
        let comp_id = comp_id.unwrap();

        let position = ENEMY_POSITION.read().unwrap().clone();
        let angle = position.angle();
        rbot::aim(comp_id, angle);
        if rbot::at_rotation(comp_id, angle, 1.0).unwrap() {
            rbot::use_component(comp_id, false);
        }
    };

    // (Hardcoded) Defensive start
    rbot::velocity(1.0, 0.0, 1.0);
    rbot::use_component(0, false);
    rbot::sleep(2.0);
    rbot::use_component(1, false);
    let angle = find_enemy().and_then(|p| Some(p.angle())).unwrap_or(0.0);
    rbot::aim(1, angle);
    rbot::await_aim(1, 0.0, 0.5);

    rbot::sleep(0.5);
    rbot::aim(2, 0.0);
    rbot::await_component(2);
    rbot::use_component(2, false);

    loop {
        // Now we need to have a defence stratergy to survive and kill incomming
        // enemies.

        // Update the estimation of the enemy robot.
        find_enemy();

        // Perform the move manouver
        Pin::new(&mut move_routine).resume(());

        // Perform the aim manouver
        Pin::new(&mut aim_routine).resume(());

        // Repair component with the least amount of hp
        Pin::new(&mut repair_routine).resume(());

        // Trigger force field as often as we can
        rbot::modules::force_field();
    }
}

fn find_enemy() -> Option<Position> {
    // let enemy_position = scan_for_average_bot_component();
    // if let Some(enemy_position) = enemy_position {
    //     *ENEMY_POSITION.write().unwrap() = enemy_position.clone();
    //     return Some(enemy_position);
    // }

    let position = laser::smart_search_for_enemy(ENEMY_POSITION.read().unwrap().angle());

    // Store the laser result for faster finding next time.
    if let Some(pos) = position {
        *ENEMY_POSITION.write().unwrap() = pos.clone();
        Some(pos)
    } else {
        None
    }
}
