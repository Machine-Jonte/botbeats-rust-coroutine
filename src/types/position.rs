use rbot::messages::{RMsgLaser, RMsgRadar};

// Struct for storing the position of an object.
#[derive(Default, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn angle(&self) -> f32 {
        rbot::conversions::xy_to_angle(self.x, self.y)
    }
}

// Automatic conversion from Radar respone.
impl From<RMsgRadar> for Position {
    fn from(value: RMsgRadar) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

// Automatic conversion from Laser response.
impl From<RMsgLaser> for Position {
    fn from(value: RMsgLaser) -> Self {
        let [x, y] = rbot::conversions::angle_to_xy(value.angle);

        Self {
            x: x * value.distance,
            y: y * value.distance,
        }
    }
}
