use crate::types::Position;
use itertools::Itertools;
use rbot;

/// 🔍 Scans for the enemy robot. Worse case the enemy is not found and this
/// will return None.
pub fn scan_for_average_bot_component() -> Option<Position> {
    let scan_msg = rbot::modules::scan().ok()?;
    let components = scan_msg
        .objects
        .into_iter()
        .filter(|o| o.tag == rbot::constants::tag::COMPONENT)
        .collect_vec();

    if components.len() == 0 {
        return None;
    }

    // Find the average position of the components.
    let x: f32 = components.iter().map(|c| c.x).sum::<f32>() / components.len() as f32;
    let y: f32 = components.iter().map(|c| c.y).sum::<f32>() / components.len() as f32;

    Some(Position { x, y })
}
