use super::util::NewAttack;
use crate::constants::*;

use std::collections::HashSet;

pub fn is_attack_valid(
    new_attack: &NewAttack,
    valid_road_paths: HashSet<(i32, i32)>,
    valid_emp_ids: HashSet<i32>,
    no_of_bombs: &i32,
) -> bool {
    let attacker_path = &new_attack.attacker_path;
    let mut attack_bomb_count = 0;
    if attacker_path.is_empty() {
        return false;
    }
    for i in 0..attacker_path.len() {
        let current_path = &attacker_path[i];
        if current_path.is_emp {
            attack_bomb_count += 1;
            if let (Some(emp_type), Some(emp_time)) = (current_path.emp_type, current_path.emp_time)
            {
                // check if emp_id is valid
                if !valid_emp_ids.contains(&emp_type) {
                    return false;
                }
                // check if emp_time is valid
                let game_minutes = GAME_MINUTES_PER_FRAME * (i as i32 + ATTACKER_RESTRICTED_FRAMES);
                if emp_time < game_minutes {
                    return false;
                }
            } else {
                return false;
            }
        }
        if !valid_road_paths.contains(&(current_path.x_coord, current_path.y_coord)) {
            return false;
        }
        if i > 0 {
            let previous_path = &attacker_path[i - 1];
            let path_difference = (previous_path.x_coord - current_path.x_coord).abs()
                + (previous_path.y_coord - current_path.y_coord).abs();
            // attacker should move every frame
            if path_difference != 1 {
                return false;
            }
        }
    }
    if attack_bomb_count > *no_of_bombs {
        return false;
    }
    true
}