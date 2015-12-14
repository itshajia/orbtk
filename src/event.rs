use super::Point;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Mouse {
        point: Point,
        left_button: bool,
        middle_button: bool,
        right_button: bool,
    },
    Unknown
}
