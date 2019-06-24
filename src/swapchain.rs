use crate::colorspace::*;
use crate::gpu::*;
use crate::vulkan::*;
use crate::*;

use libplacebo_sys::*;

use std::default::Default;

pub type VkPresentMode = VkPresentModeKHR;
pub type VkColorSpace = VkColorSpaceKHR;

create_complete_struct!(
    SurfaceFormat,
    surface_format,
    VkSurfaceFormatKHR,
    (format, colorSpace),
    (&VkFormat, &VkColorSpace),
    (
        VkFormat::VK_FORMAT_UNDEFINED as VkFormat,
        VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR as VkColorSpaceKHR,
    ),
    (*format as VkFormat, *colorSpace as VkColorSpace)
);

create_complete_struct!(
    SwapchainParams,
    sw_params,
    pl_vulkan_swapchain_params,
    (surface, present_mode, surface_format, swapchain_depth),
    (u64, &VkPresentMode, &SurfaceFormat, usize),
    (
        surface as VkSurfaceKHR,
        *present_mode as VkPresentMode,
        surface_format.surface_format,
        swapchain_depth as i32,
    )
);

impl Default for SwapchainParams {
    fn default() -> Self {
        let surf_default: SurfaceFormat = Default::default();
        let sw_params = pl_vulkan_swapchain_params {
            surface: 0 as VkSurfaceKHR,
            present_mode: VkPresentModeKHR::VK_PRESENT_MODE_IMMEDIATE_KHR,
            surface_format: surf_default.surface_format,
            swapchain_depth: 3,
        };
        SwapchainParams { sw_params }
    }
}

create_complete_struct!(
    SwapchainFrame,
    frame,
    pl_swapchain_frame,
    (fbo, flipped, color_repr, color_space),
    (&Tex, bool, &ColorRepr, &ColorSpace),
    (
        0 as *const pl_tex,
        false,
        ColorRepr::to_color_repr(&ColorReprs::Unknown).internal_object(),
        ColorSpace::to_color_space(&ColorSpaces::Unknown).internal_object(),
    ),
    (
        fbo.get_ptr(),
        flipped as bool,
        color_repr.internal_object(),
        color_space.internal_object(),
    )
);

get_ptr!(SwapchainFrame, frame, pl_swapchain_frame);

pub struct Swapchain {
    sw: *const pl_swapchain,
}

impl Swapchain {
    pub fn new(vk: &Vulkan, params: &SwapchainParams) -> Self {
        let sw = unsafe {
            pl_vulkan_create_swapchain(vk.get_ptr(), &params.sw_params)
        };
        assert!(!sw.is_null());

        Swapchain { sw }
    }

    pub fn latency(&self) -> usize {
        unsafe { pl_swapchain_latency(self.sw) as usize }
    }

    pub fn resize(&self, width: usize, height: usize) -> (usize, usize) {
        let mut w = width as i32;
        let mut h = height as i32;
        let ok = unsafe { pl_swapchain_resize(self.sw, &mut w, &mut h) };
        assert!(ok);
        (w as usize, h as usize)
    }

    pub fn start_frame(&self, frame: &mut SwapchainFrame) -> bool {
        unsafe { pl_swapchain_start_frame(self.sw, &mut frame.frame) }
    }

    pub fn submit_frame(&self) {
        let ok = unsafe { pl_swapchain_submit_frame(self.sw) };
        assert!(ok);
    }

    pub fn swap_buffers(&self) {
        unsafe {
            pl_swapchain_swap_buffers(self.sw);
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            pl_swapchain_destroy(&mut self.sw);
        }
    }
}
