/// Resource over relevant input.
pub struct Input {
    pub mouse_location: (i32, i32),
    pub mouse_click: Option<(i32, i32)>,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}

impl Input {
    pub fn vertical_axis(&self) -> f64 {
        let up = if self.up { 1.0 } else { 0.0 };
        let down = if self.down { -1.0 } else { 0.0 };

        up + down
    }

    pub fn horizontal_axis(&self) -> f64 {
        let left = if self.left { -1.0 } else { 0.0 };
        let right = if self.right { 1.0 } else { 0.0 };

        left + right
    }
}

impl Default for Input {
    fn default() -> Self {
        Input {
            mouse_location: (0, 0),
            mouse_click: None,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}
