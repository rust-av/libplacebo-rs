use crate::*;

use libplacebo_sys::*;

use std::ffi::c_void;
use std::ptr::null;

create_enum!(
    AlphaMode,
    pl_alpha_mode,
    (ALPHA_UNKNOWN, ALPHA_INDEPENDENT, ALPHA_PREMULTIPLIED)
);

create_enum!(
    ColorSystem,
    pl_color_system,
    (
        COLOR_SYSTEM_UNKNOWN,
        COLOR_SYSTEM_BT_601,
        COLOR_SYSTEM_BT_709,
        COLOR_SYSTEM_SMPTE_240M,
        COLOR_SYSTEM_BT_2020_NC,
        COLOR_SYSTEM_BT_2020_C,
        COLOR_SYSTEM_BT_2100_PQ,
        COLOR_SYSTEM_BT_2100_HLG,
        COLOR_SYSTEM_YCGCO,
        COLOR_SYSTEM_RGB,
        COLOR_SYSTEM_XYZ,
        COLOR_SYSTEM_COUNT,
    )
);

create_enum!(
    ColorPrimaries,
    pl_color_primaries,
    (
        COLOR_PRIM_UNKNOWN,
        COLOR_PRIM_BT_601_525,
        COLOR_PRIM_BT_601_625,
        COLOR_PRIM_BT_709,
        COLOR_PRIM_BT_470M,
        COLOR_PRIM_BT_2020,
        COLOR_PRIM_APPLE,
        COLOR_PRIM_ADOBE,
        COLOR_PRIM_PRO_PHOTO,
        COLOR_PRIM_CIE_1931,
        COLOR_PRIM_DCI_P3,
        COLOR_PRIM_DISPLAY_P3,
        COLOR_PRIM_V_GAMUT,
        COLOR_PRIM_S_GAMUT,
        COLOR_PRIM_COUNT,
    )
);

create_enum!(
    ColorTransfer,
    pl_color_transfer,
    (
        COLOR_TRC_UNKNOWN,
        COLOR_TRC_BT_1886,
        COLOR_TRC_SRGB,
        COLOR_TRC_LINEAR,
        COLOR_TRC_GAMMA18,
        COLOR_TRC_GAMMA22,
        COLOR_TRC_GAMMA28,
        COLOR_TRC_PRO_PHOTO,
        COLOR_TRC_PQ,
        COLOR_TRC_HLG,
        COLOR_TRC_V_LOG,
        COLOR_TRC_S_LOG1,
        COLOR_TRC_S_LOG2,
        COLOR_TRC_COUNT,
    )
);

create_enum!(
    ColorLight,
    pl_color_light,
    (
        COLOR_LIGHT_UNKNOWN,
        COLOR_LIGHT_DISPLAY,
        COLOR_LIGHT_SCENE_HLG,
        COLOR_LIGHT_SCENE_709_1886,
        COLOR_LIGHT_SCENE_1_2,
        COLOR_LIGHT_COUNT,
    )
);

create_enum!(
    ColorLevels,
    pl_color_levels,
    (
        COLOR_LEVELS_UNKNOWN,
        COLOR_LEVELS_TV,
        COLOR_LEVELS_PC,
        COLOR_LEVELS_COUNT,
    )
);

create_enum!(
    RenderingIntent,
    pl_rendering_intent,
    (
        INTENT_PERCEPTUAL,
        INTENT_RELATIVE_COLORIMETRIC,
        INTENT_SATURATION,
        INTENT_ABSOLUTE_COLORIMETRIC,
    )
);

create_complete_struct!(
    BitEncoding,
    bit_encoding,
    pl_bit_encoding,
    (sample_depth, color_depth, bit_shift),
    (usize, usize, usize),
    (0, 0, 0),
    (sample_depth as i32, color_depth as i32, bit_shift as i32)
);

impl BitEncoding {
    pub fn equal(&self, b: &BitEncoding) -> bool {
        unsafe { pl_bit_encoding_equal(&self.bit_encoding, &b.bit_encoding) }
    }
}

pub enum ColorReprs {
    Unknown,
    Rgb,
    Sdtv,
    Hdtv,
    Uhdtv,
    Jpeg,
}

create_struct!(
    ColorRepr,
    color_repr,
    pl_color_repr,
    (sys, levels, alpha, bits),
    (&ColorSystem, &ColorLevels, &AlphaMode, &BitEncoding),
    (
        ColorSystem::to_pl_color_system(sys),
        ColorLevels::to_pl_color_levels(levels),
        AlphaMode::to_pl_alpha_mode(alpha),
        bits.bit_encoding as pl_bit_encoding,
    )
);

impl Default for ColorRepr {
    fn default() -> Self {
        let bits: BitEncoding = Default::default();
        let color_repr = pl_color_repr {
            sys: pl_color_system::PL_COLOR_SYSTEM_UNKNOWN,
            levels: pl_color_levels::PL_COLOR_LEVELS_UNKNOWN,
            alpha: pl_alpha_mode::PL_ALPHA_UNKNOWN,
            bits: bits.bit_encoding,
        };
        ColorRepr { color_repr }
    }
}

impl ColorRepr {
    pub fn color_repr(repr: &ColorReprs) -> Self {
        unsafe {
            let color_repr = match repr {
                ColorReprs::Unknown => pl_color_repr_unknown,
                ColorReprs::Rgb => pl_color_repr_rgb,
                ColorReprs::Sdtv => pl_color_repr_sdtv,
                ColorReprs::Hdtv => pl_color_repr_hdtv,
                ColorReprs::Uhdtv => pl_color_repr_uhdtv,
                ColorReprs::Jpeg => pl_color_repr_jpeg,
            };
            ColorRepr { color_repr }
        }
    }

    pub(crate) fn from_pl(color_repr: pl_color_repr) -> Self {
        ColorRepr { color_repr }
    }
}

internal_object!(ColorRepr, color_repr, pl_color_repr);

pub enum ColorSpaces {
    Unknown,
    Srgb,
    Bt709,
    Hdr10,
    Bt2020Hlg,
    Monitor,
}

create_complete_struct!(
    ColorSpace,
    color_space,
    pl_color_space,
    (primaries, transfer, light, sig_peak, sig_avg, sig_scale),
    (&ColorPrimaries, &ColorTransfer, &ColorLight, f32, f32, f32),
    (
        pl_color_primaries::PL_COLOR_PRIM_UNKNOWN,
        pl_color_transfer::PL_COLOR_TRC_UNKNOWN,
        pl_color_light::PL_COLOR_LIGHT_UNKNOWN,
        0.0,
        0.0,
        0.0
    ),
    (
        ColorPrimaries::to_pl_color_primaries(primaries),
        ColorTransfer::to_pl_color_transfer(transfer),
        ColorLight::to_pl_color_light(light),
        sig_peak as f32,
        sig_avg as f32,
        sig_scale as f32
    )
);

impl ColorSpace {
    pub fn color_space(color: &ColorSpaces) -> Self {
        unsafe {
            let color_space = match color {
                ColorSpaces::Unknown => pl_color_space_unknown,
                ColorSpaces::Srgb => pl_color_space_srgb,
                ColorSpaces::Bt709 => pl_color_space_bt709,
                ColorSpaces::Hdr10 => pl_color_space_hdr10,
                ColorSpaces::Bt2020Hlg => pl_color_space_bt2020_hlg,
                ColorSpaces::Monitor => pl_color_space_monitor,
            };

            ColorSpace { color_space }
        }
    }

    pub(crate) fn from_pl(color_space: pl_color_space) -> Self {
        ColorSpace { color_space }
    }
}

internal_object!(ColorSpace, color_space, pl_color_space);

pub enum Vision {
    Normal,
    Protanomaly,
    Protanopia,
    Deuteranomaly,
    Deuteranopia,
    Tritanomaly,
    Tritanopia,
    Monochromacy,
    Achromatopsia,
}

impl Vision {
    pub fn to_cone_params(&self) -> pl_cone_params {
        unsafe {
            match self {
                Vision::Normal => pl_vision_normal,
                Vision::Protanomaly => pl_vision_protanomaly,
                Vision::Protanopia => pl_vision_protanopia,
                Vision::Deuteranomaly => pl_vision_deuteranomaly,
                Vision::Deuteranopia => pl_vision_deuteranopia,
                Vision::Tritanomaly => pl_vision_tritanomaly,
                Vision::Tritanopia => pl_vision_tritanopia,
                Vision::Monochromacy => pl_vision_monochromacy,
                Vision::Achromatopsia => pl_vision_achromatopsia,
            }
        }
    }
}

create_struct!(
    ColorAdjustment,
    color_adjustment,
    pl_color_adjustment,
    (brightness, contrast, saturation, hue, gamma),
    (f32, f32, f32, f32, f32),
    (
        brightness as f32,
        contrast as f32,
        saturation as f32,
        hue as f32,
        gamma as f32
    )
);

default_struct!(ColorAdjustment, color_adjustment, unsafe {
    pl_color_adjustment_neutral
});

get_ptr!(ColorAdjustment, color_adjustment, pl_color_adjustment);

pub struct IccProfile {
    icc_profile: pl_icc_profile,
    data: Vec<u8>,
}

impl Default for IccProfile {
    fn default() -> Self {
        let icc_profile = pl_icc_profile {
            signature: 0 as u64,
            data: null(),
            len: 0 as usize,
        };

        IccProfile {
            icc_profile,
            data: Vec::new(),
        }
    }
}

impl IccProfile {
    pub fn new(signature: usize, data: &[u8]) -> Self {
        let data_i = data.to_owned();
        let icc_profile = pl_icc_profile {
            signature: signature as u64,
            data: data_i.as_ptr() as *const c_void,
            len: data.len(),
        };

        IccProfile {
            icc_profile,
            data: data_i,
        }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.data = data.to_owned();
        self.icc_profile.data = self.data.as_ptr() as *const c_void;
        self.icc_profile.len = data.len();
    }

    pub fn set_signature(&mut self, signature: usize) {
        self.icc_profile.signature = signature as u64;
    }

    pub fn is_equal(&self, icc_profile: &IccProfile) -> bool {
        unsafe {
            pl_icc_profile_equal(&self.icc_profile, &icc_profile.icc_profile)
        }
    }
}

internal_object!(IccProfile, icc_profile, pl_icc_profile);
