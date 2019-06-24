use crate::*;

use libplacebo_sys::*;

create_complete_struct!(
    Rect2D,
    rect2d,
    pl_rect2d,
    (x0, y0, x1, y1),
    (usize, usize, usize, usize),
    (0, 0, 0, 0),
    (x0 as i32, y0 as i32, x1 as i32, y1 as i32)
);

internal_object!(Rect2D, rect2d, pl_rect2d);

create_complete_struct!(
    Rect2DF,
    rect2df,
    pl_rect2df,
    (x0, y0, x1, y1),
    (f32, f32, f32, f32),
    (0.0, 0.0, 0.0, 0.0),
    (x0 as f32, y0 as f32, x1 as f32, y1 as f32)
);

internal_object!(Rect2DF, rect2df, pl_rect2df);

create_complete_struct!(
    Rect3D,
    rect3d,
    pl_rect3d,
    (x0, y0, z0, x1, y1, z1),
    (i32, i32, i32, i32, i32, i32),
    (0, 0, 0, 0, 0, 0),
    (x0 as i32, y0 as i32, z0 as i32, x1 as i32, y1 as i32, z1 as i32)
);

create_complete_struct!(
    Rect3DF,
    rect3df,
    pl_rect3df,
    (x0, y0, z0, x1, y1, z1),
    (f32, f32, f32, f32, f32, f32),
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    (x0 as f32, y0 as f32, z0 as f32, x1 as f32, y1 as f32, z1 as f32)
);
