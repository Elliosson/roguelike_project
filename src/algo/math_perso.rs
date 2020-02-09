use std::sync::atomic::{AtomicUsize, Ordering};

pub fn add_or_zero(u: u32, i: i32) -> u32 {
    if (u as i32 + i) < 0 {
        return 0;
    } else {
        return (u as i32 + i) as u32;
    }
}

//generate an unique id Id
pub fn _unique_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn dummy_flee(my_pos_x: i32, my_pos_y: i32, enemy_pos_x: i32, enemy_pos_y: i32) -> (i32, i32) {
    let y_dist = my_pos_y - enemy_pos_y;
    let x_dist = my_pos_x - enemy_pos_x;
    let ret;
    if y_dist.abs() > x_dist.abs() {
        if my_pos_y > enemy_pos_y {
            ret = (my_pos_x, my_pos_y + 1)
        } else {
            ret = (my_pos_x, my_pos_y - 1)
        }
    } else {
        if my_pos_x > enemy_pos_x {
            ret = (my_pos_x + 1, my_pos_y)
        } else {
            ret = (my_pos_x - 1, my_pos_y)
        }
    }
    ret
}
