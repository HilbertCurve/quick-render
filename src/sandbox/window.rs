pub const DEFAULT_WH: (usize, usize) = (680, 400);

static mut WINDOW_W: usize = DEFAULT_WH.0;
static mut WINDOW_H: usize = DEFAULT_WH.1;

pub fn get_width_height() -> (usize, usize) {
    unsafe { (WINDOW_W, WINDOW_H) }
}

pub fn set_width_height(width: usize, height: usize) {
    unsafe {
        WINDOW_W = width;
        WINDOW_H = height;
    }
}