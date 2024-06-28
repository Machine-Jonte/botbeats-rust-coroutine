use crate::types::Position;

/// 🔫 Find the enemy using the laser component. If the enemy is not found, the
/// function returns `None`.
///
/// 🔍 The laser will search a bit smarter by starting in the center and moving
/// outwards. The start search angle is provided to the function for quicker
/// search.
pub fn smart_search_for_enemy(start_search_angle: f32) -> Option<Position> {
    let search_angle = start_search_angle;
    let mut offset_value = 0.;
    let diff = 3.;
    let mut sign = -1.;

    // Have a bound on the search to avoid searching forever.
    offset_value -= diff;
    for _ in 0..50 {
        sign *= -1.;
        offset_value += diff;

        let angle = search_angle + offset_value * sign;

        let laser_msg = rbot::modules::laser(angle).ok()?;
        if laser_msg.tag == rbot::constants::tag::COMPONENT {
            return Some(laser_msg.into());
        }
    }

    None
}
