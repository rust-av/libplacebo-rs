use crate::colorspace::*;
use crate::*;

use libplacebo_sys::*;

create_enum!(
    ToneMappingAlgorithm,
    pl_tone_mapping_algorithm,
    (
        TONE_MAPPING_CLIP,
        TONE_MAPPING_MOBIUS,
        TONE_MAPPING_REINHARD,
        TONE_MAPPING_HABLE,
        TONE_MAPPING_GAMMA,
        TONE_MAPPING_LINEAR,
    )
);

create_enum!(
    DitherMethod,
    pl_dither_method,
    (
        DITHER_BLUE_NOISE,
        DITHER_ORDERED_LUT,
        DITHER_ORDERED_FIXED,
        DITHER_WHITE_NOISE,
    )
);

create_struct!(
    SigmoidParams,
    sigmoid_params,
    pl_sigmoid_params,
    (center, slope),
    (f32, f32),
    (center as f32, slope as f32)
);

default_struct!(SigmoidParams, sigmoid_params, unsafe {
    pl_sigmoid_default_params
});

get_ptr!(SigmoidParams, sigmoid_params, pl_sigmoid_params);

create_struct!(
    PeakDetectParams,
    detect_params,
    pl_peak_detect_params,
    (smoothing_period, scene_threshold_low, scene_threshold_high),
    (f32, f32, f32),
    (
        smoothing_period as f32,
        scene_threshold_low as f32,
        scene_threshold_high as f32
    )
);

default_struct!(PeakDetectParams, detect_params, unsafe {
    pl_peak_detect_default_params
});

get_ptr!(PeakDetectParams, detect_params, pl_peak_detect_params);

set_struct!(ColorMapParams, colormap_params, pl_color_map_params);

default_struct!(ColorMapParams, colormap_params, unsafe {
    pl_color_map_default_params
});

set_params!(
    ColorMapParams,
    colormap_params,
    (
        intent,
        tone_mapping_algo,
        tone_mapping_param,
        desaturation_strength,
        desaturation_exponent,
        desaturation_base,
        max_boost,
        gamut_warning
    ),
    (
        &RenderingIntent,
        &ToneMappingAlgorithm,
        f32,
        f32,
        f32,
        f32,
        f32,
        bool
    ),
    (
        RenderingIntent::to_pl_rendering_intent(intent),
        ToneMappingAlgorithm::to_pl_tone_mapping_algorithm(tone_mapping_algo),
        tone_mapping_param as f32,
        desaturation_strength as f32,
        desaturation_exponent as f32,
        desaturation_base as f32,
        max_boost as f32,
        gamut_warning as bool
    )
);

get_ptr!(ColorMapParams, colormap_params, pl_color_map_params);

create_struct!(
    DitherParams,
    dither_params,
    pl_dither_params,
    (method, lut_size, temporal),
    (&DitherMethod, usize, bool),
    (
        DitherMethod::to_pl_dither_method(method),
        lut_size as i32,
        temporal as bool
    )
);

default_struct!(DitherParams, dither_params, unsafe {
    pl_dither_default_params
});

get_ptr!(DitherParams, dither_params, pl_dither_params);

create_struct!(
    Lut3DParams,
    lut3d_params,
    pl_3dlut_params,
    (intent, size_r, size_g, size_b),
    (&RenderingIntent, usize, usize, usize),
    (
        RenderingIntent::to_pl_rendering_intent(intent),
        size_r as usize,
        size_g as usize,
        size_b as usize
    )
);

default_struct!(Lut3DParams, lut3d_params, unsafe {
    pl_3dlut_default_params
});

get_ptr!(Lut3DParams, lut3d_params, pl_3dlut_params);
