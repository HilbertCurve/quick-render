static mut WINDOW_W: usize = 0;
static mut WINDOW_H: usize = 0;

pub fn get_width_height() -> (usize, usize) {
    unsafe { (WINDOW_W, WINDOW_H) }
}

pub fn set_width_height(width: usize, height: usize) {
    unsafe {
        WINDOW_W = width;
        WINDOW_H = height;
    }
}