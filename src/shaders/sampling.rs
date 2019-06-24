use crate::*;

use libplacebo_sys::*;

create_struct!(
    DebandParams,
    deband_params,
    pl_deband_params,
    (iterations, threshold, radius, grain),
    (usize, f32, f32, f32),
    (
        iterations as i32,
        threshold as f32,
        radius as f32,
        grain as f32
    )
);

default_struct!(DebandParams, deband_params, unsafe {
    pl_deband_default_params
});

get_ptr!(DebandParams, deband_params, pl_deband_params);
