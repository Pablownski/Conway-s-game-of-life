use crate::framebuffer::{Framebuffer, ALIVE};

fn place(fb: &mut Framebuffer, ox: usize, oy: usize, cells: &[(usize, usize)]) {
    for &(cx, cy) in cells {
        fb.point(ox + cx, oy + cy, ALIVE);
    }
}

pub fn glider(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
}

pub fn blinker(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (2, 0)]);
}

pub fn toad(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]);
}

pub fn beacon(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)],
    );
}

pub fn block(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (0, 1), (1, 1)]);
}

pub fn beehive(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)]);
}

pub fn loaf(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)],
    );
}

pub fn boat(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)]);
}

pub fn tub(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(1, 0), (0, 1), (2, 1), (1, 2)]);
}

pub fn r_pentomino(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)]);
}

pub fn diehard(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(6, 0), (0, 1), (1, 1), (1, 2), (5, 2), (6, 2), (7, 2)],
    );
}

pub fn acorn(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)],
    );
}

pub fn lwss(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (0, 1),
            (4, 1),
            (4, 2),
            (0, 3),
            (3, 3),
        ],
    );
}

pub fn mwss(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (2, 0),
            (0, 1),
            (3, 1),
            (4, 2),
            (0, 3),
            (4, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
        ],
    );
}

pub fn hwss(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (2, 0),
            (3, 0),
            (0, 1),
            (4, 1),
            (5, 2),
            (0, 3),
            (5, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
        ],
    );
}

pub fn pentadecathlon(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (2, 0),
            (7, 0),
            (0, 1),
            (1, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (8, 1),
            (9, 1),
            (2, 2),
            (7, 2),
        ],
    );
}


pub fn pulsar(fb: &mut Framebuffer, ox: usize, oy: usize) {
    let rows = [0, 5, 7, 12];
    let cols = [2, 3, 4, 8, 9, 10];
    for &r in &rows {
        for &c in &cols {
            fb.point(ox + c, oy + r, ALIVE);
        }
    }
    for &c in &rows {
        for &r in &cols {
            fb.point(ox + c, oy + r, ALIVE);
        }
    }
}


pub fn gosper_glider_gun(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (24, 0),
            (22, 1),
            (24, 1),
            (12, 2),
            (13, 2),
            (20, 2),
            (21, 2),
            (34, 2),
            (35, 2),
            (11, 3),
            (15, 3),
            (20, 3),
            (21, 3),
            (34, 3),
            (35, 3),
            (0, 4),
            (1, 4),
            (10, 4),
            (16, 4),
            (20, 4),
            (21, 4),
            (0, 5),
            (1, 5),
            (10, 5),
            (14, 5),
            (16, 5),
            (17, 5),
            (22, 5),
            (24, 5),
            (10, 6),
            (16, 6),
            (24, 6),
            (11, 7),
            (15, 7),
            (12, 8),
            (13, 8),
        ],
    );
}


pub fn load_initial_pattern(fb: &mut Framebuffer) {
   
    gosper_glider_gun(fb, 2, 2);
    pulsar(fb, 45, 2);
    lwss(fb, 62, 5);
    lwss(fb, 70, 5);
    pentadecathlon(fb, 80, 3);
    diehard(fb, 95, 3);
    acorn(fb, 105, 8);

   
    r_pentomino(fb, 5, 20);
    beacon(fb, 15, 20);
    toad(fb, 25, 20);
    blinker(fb, 35, 20);
    block(fb, 45, 20);
    beehive(fb, 55, 20);
    glider(fb, 65, 20);
    lwss(fb, 75, 20);
    pentadecathlon(fb, 90, 20);
    pulsar(fb, 100, 25);

    
    diehard(fb, 5, 35);
    acorn(fb, 18, 35);
    r_pentomino(fb, 30, 35);
    beacon(fb, 40, 35);
    toad(fb, 50, 35);
    blinker(fb, 60, 35);
    block(fb, 70, 35);
    beehive(fb, 78, 35);
    glider(fb, 88, 35);

    
    gosper_glider_gun(fb, 2, 50);
    lwss(fb, 45, 52);
    lwss(fb, 55, 52);
    pentadecathlon(fb, 65, 52);
    pulsar(fb, 80, 50);
    diehard(fb, 95, 55);
    acorn(fb, 105, 60);

   
    r_pentomino(fb, 5, 70);
    beacon(fb, 15, 70);
    toad(fb, 25, 70);
    blinker(fb, 35, 70);
    block(fb, 45, 70);
    beehive(fb, 55, 70);
    glider(fb, 65, 70);
    lwss(fb, 75, 70);
    pentadecathlon(fb, 90, 75);
    pulsar(fb, 100, 80);

    diehard(fb, 5, 85);
    acorn(fb, 18, 85);
    beacon(fb, 30, 88);
    toad(fb, 40, 88);
    blinker(fb, 50, 88);
    block(fb, 58, 88);
    beehive(fb, 66, 88);
    glider(fb, 76, 88);


    loaf(fb, 5, 96);
    boat(fb, 15, 96);
    tub(fb, 25, 96);
    mwss(fb, 40, 96);
    hwss(fb, 55, 96);
}
