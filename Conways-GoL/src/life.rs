use crate::framebuffer::{Framebuffer, ALIVE, DEAD};

fn count_neighbors(fb: &Framebuffer, x: usize, y: usize) -> u32 {
    let w = fb.width as i32;
    let h = fb.height as i32;
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = ((x as i32 + dx + w) % w) as usize;
            let ny = ((y as i32 + dy + h) % h) as usize;
            if fb.get_color(nx, ny) == ALIVE {
                count += 1;
            }
        }
    }

    count
}

pub fn step(current: &Framebuffer, next: &mut Framebuffer) {
    for y in 0..current.height {
        for x in 0..current.width {
            let alive = current.get_color(x, y) == ALIVE;
            let neighbors = count_neighbors(current, x, y);

            let will_live = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };

            next.point(x, y, if will_live { ALIVE } else { DEAD });
        }
    }
}
