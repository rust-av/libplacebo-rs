use crate::colorspace::*;
use crate::common::*;
use crate::context::*;
use crate::filter::*;
use crate::gpu::*;
use crate::shaders::colorspace::*;
use crate::shaders::sampling::*;
use crate::swapchain::*;
use crate::*;

use libplacebo_sys::*;

use std::ptr::null;

create_enum!(
    OverlayMode,
    pl_overlay_mode,
    (OVERLAY_NORMAL, OVERLAY_MONOCHROME,)
);

macro_rules! overlays {
    ($id:ident) => {
        pub fn set_overlays(&mut self, overlays: &[Overlay]) {
            self.overlays = overlays.iter().map(|v| v.overlay).collect();
            self.$id.overlays = self.overlays.as_ptr();
            self.$id.num_overlays = overlays.len() as i32;
        }
    }
}

create_complete_struct!(
    Plane,
    plane,
    pl_plane,
    (texture, components, component_mapping, shift_x, shift_y),
    (&Tex, usize, [i32; 4], f32, f32),
    (null(), 0, [0; 4], 0.0, 0.0),
    (
        texture.get_ptr(),
        components as i32,
        component_mapping as [i32; 4],
        shift_x as f32,
        shift_y as f32,
    )
);

impl Plane {
    pub fn width(&self) -> usize {
        unsafe { (*self.plane.texture).params.w as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { (*self.plane.texture).params.h as usize }
    }
    pub(crate) fn get_mut_ptr(&mut self) -> *mut pl_plane {
        &mut self.plane
    }
}

create_struct!(
    Overlay,
    overlay,
    pl_overlay,
    (plane, rect, mode, base_color, repr, color),
    (
        &Plane,
        &Rect2D,
        &OverlayMode,
        &[f32; 3],
        &ColorRepr,
        &ColorSpace,
    ),
    (
        plane.plane,
        rect.internal_object(),
        OverlayMode::to_pl_overlay_mode(mode),
        *base_color as [f32; 3],
        repr.internal_object(),
        color.internal_object(),
    )
);

pub struct Image {
    img: pl_image,
    overlays: Vec<pl_overlay>,
}

impl Default for Image {
    fn default() -> Self {
        let plane: Plane = Default::default();
        let pl_planes: [pl_plane; 4] =
            [plane.plane, plane.plane, plane.plane, plane.plane];
        let icc_profile: IccProfile = Default::default();
        let rect: Rect2DF = Default::default();
        let img = pl_image {
            signature: 0,
            num_planes: 0,
            planes: pl_planes,
            repr: ColorRepr::color_repr(&ColorReprs::Unknown)
                .internal_object(),
            color: ColorSpace::color_space(&ColorSpaces::Unknown)
                .internal_object(),
            profile: icc_profile.internal_object(),
            width: 0,
            height: 0,
            src_rect: rect.internal_object(),
            overlays: null(),
            num_overlays: 0,
        };

        Image {
            img,
            overlays: Vec::new(),
        }
    }
}

set_params!(
    Image,
    img,
    (signature, num_planes, repr, color, profile, width, height, src_rect),
    (
        usize,
        usize,
        &ColorRepr,
        &ColorSpace,
        &IccProfile,
        usize,
        usize,
        &Rect2DF,
    ),
    (
        signature as u64,
        num_planes as i32,
        repr.internal_object(),
        color.internal_object(),
        profile.internal_object(),
        width as i32,
        height as i32,
        src_rect.internal_object(),
    )
);

impl Image {
    pub fn repr(&self) -> ColorRepr {
        ColorRepr::from_pl(self.img.repr)
    }

    pub fn color(&self) -> ColorSpace {
        ColorSpace::from_pl(self.img.color)
    }

    pub fn set_planes(&mut self, planes: &[&Plane; 4]) {
        let pl_planes: [pl_plane; 4] = [
            planes[0].clone().plane,
            planes[1].clone().plane,
            planes[2].clone().plane,
            planes[3].clone().plane,
        ];
        self.img.planes = pl_planes;
    }

    overlays!(img);
}

pub struct RenderTarget {
    target: pl_render_target,
    overlays: Vec<pl_overlay>,
}

impl Default for RenderTarget {
    fn default() -> Self {
        let icc_profile: IccProfile = Default::default();
        let rect: Rect2D = Default::default();
        let target = pl_render_target {
            fbo: null(),
            dst_rect: rect.internal_object(),
            repr: ColorRepr::color_repr(&ColorReprs::Unknown)
                .internal_object(),
            color: ColorSpace::color_space(&ColorSpaces::Unknown)
                .internal_object(),
            profile: icc_profile.internal_object(),
            overlays: null(),
            num_overlays: 0,
        };

        RenderTarget {
            target,
            overlays: Vec::new(),
        }
    }
}

set_params!(
    RenderTarget,
    target,
    (fbo, dst_rect, repr, color, profile),
    (&Tex, &Rect2D, &ColorRepr, &ColorSpace, &IccProfile),
    (
        fbo.get_ptr(),
        dst_rect.internal_object(),
        repr.internal_object(),
        color.internal_object(),
        profile.internal_object(),
    )
);

impl RenderTarget {
    pub fn render_target_from_swapchain(&mut self, frame: &SwapchainFrame) {
        unsafe {
            pl_render_target_from_swapchain(&mut self.target, frame.get_ptr());
        }
    }

    overlays!(target);
}

set_struct!(RenderParams, params, pl_render_params);

default_struct!(RenderParams, params, unsafe { pl_render_default_params });

set_params!(
    RenderParams,
    params,
    (
        upscaler,
        downscaler,
        lut_entries,
        antiringing_strength,
        frame_mixer,
        deband_params,
        sigmoid_params,
        color_adjustment,
        peak_detect_params,
        color_map_params,
        dither_params,
        lut3d_params,
        cone_params,
        skip_anti_aliasing,
        polar_cutoff,
        disable_overlay_sampling,
        allow_delayed_peak_detect,
        skip_redraw_caching,
        disable_linear_scaling,
        disable_builtin_scalers,
        force_3dlut,
    ),
    (
        &FilterConfig,
        &FilterConfig,
        usize,
        f32,
        &FilterConfig,
        &DebandParams,
        &SigmoidParams,
        &ColorAdjustment,
        &PeakDetectParams,
        &ColorMapParams,
        &DitherParams,
        &Lut3DParams,
        &Vision,
        bool,
        f32,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
    ),
    (
        upscaler.get_ptr(),
        downscaler.get_ptr(),
        lut_entries as i32,
        antiringing_strength as f32,
        frame_mixer.get_ptr(),
        deband_params.get_ptr(),
        sigmoid_params.get_ptr(),
        color_adjustment.get_ptr(),
        peak_detect_params.get_ptr(),
        color_map_params.get_ptr(),
        dither_params.get_ptr(),
        lut3d_params.get_ptr(),
        &Vision::to_cone_params(cone_params),
        skip_anti_aliasing as bool,
        polar_cutoff as f32,
        disable_overlay_sampling as bool,
        allow_delayed_peak_detect as bool,
        skip_redraw_caching as bool,
        disable_linear_scaling as bool,
        disable_builtin_scalers as bool,
        force_3dlut as bool,
    )
);

pub struct Renderer {
    rr: *mut pl_renderer,
}

impl Renderer {
    pub fn new(ctx: &Context, gpu: &Gpu) -> Self {
        let rr =
            unsafe { pl_renderer_create(ctx.get_mut_ptr(), gpu.get_ptr()) };
        assert!(!rr.is_null());

        Renderer { rr }
    }

    pub fn flush_cache(&mut self) {
        unsafe {
            pl_renderer_flush_cache(self.rr);
        }
    }

    pub fn render_image(
        &self,
        image: &Image,
        target: &RenderTarget,
        params: &RenderParams,
    ) -> bool {
        unsafe {
            pl_render_image(
                self.rr,
                &image.img,
                &target.target,
                &params.params,
            )
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            pl_renderer_destroy(&mut self.rr);
        }
    }
}
