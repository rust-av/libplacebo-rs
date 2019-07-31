use crate::context::*;
use crate::gpu::*;
use crate::*;

use libplacebo_sys::*;

use std::ffi::CString;
use std::ptr::null_mut;

macro_rules! init_data {
    ($first:ident, $second:ident) => {
        let $first = Data {
            c_str: Vec::new(),
            c_ptr: Vec::new(),
            c_sli: null_mut(),
        };
        let $second = Data {
            c_str: Vec::new(),
            c_ptr: Vec::new(),
            c_sli: null_mut(),
        };
    };
}

macro_rules! define_data {
    ($vec:ident, $c_vec:expr) => {
        $c_vec.c_str =
            $vec.iter().map(|arg| CString::new(*arg).unwrap()).collect();
        $c_vec.c_ptr = $c_vec.c_str.iter().map(|arg| arg.as_ptr()).collect();
        $c_vec.c_sli = $c_vec.c_ptr.as_mut_ptr();
    };
}

macro_rules! extensions {
    ($param:ident) => {
        pub fn set_extensions(&mut self, ext: &[&'static str]) {
            define_data!(ext, self.c_ext);
            self.$param.extensions = self.c_ext.c_sli;
            self.$param.num_extensions = ext.len() as i32;
        }

        pub fn set_opt_extensions(&mut self, opt: &[&'static str]) {
            define_data!(opt, self.c_opt);
            self.$param.opt_extensions = self.c_opt.c_sli;
            self.$param.num_opt_extensions = opt.len() as i32;
        }
    }
}

struct Data {
    c_str: Vec<CString>,
    c_ptr: Vec<*const i8>,
    c_sli: *mut *const i8,
}

pub struct VulkanInstanceParams {
    vk_inst_params: pl_vk_inst_params,
    c_ext: Data,
    c_opt: Data,
}

impl Default for VulkanInstanceParams {
    fn default() -> Self {
        init_data!(c_ext, c_opt);
        VulkanInstanceParams {
            vk_inst_params: unsafe { pl_vk_inst_default_params },
            c_ext,
            c_opt,
        }
    }
}

impl VulkanInstanceParams {
    pub fn set_debug(&mut self, debug: bool) {
        self.vk_inst_params.debug = debug;
    }

    extensions!(vk_inst_params);
}

pub struct VulkanInstance {
    inst: *const pl_vk_inst,
}

impl VulkanInstance {
    pub fn new(ctx: &Context, params: &VulkanInstanceParams) -> Self {
        let inst = unsafe {
            pl_vk_inst_create(ctx.get_mut_ptr(), &params.vk_inst_params)
        };
        assert!(!inst.is_null());

        VulkanInstance { inst }
    }

    pub fn instance(&self) -> usize {
        unsafe { (*self.inst).instance as usize }
    }
}

impl Drop for VulkanInstance {
    fn drop(&mut self) {
        unsafe {
            pl_vk_inst_destroy(&mut self.inst);
        }
    }
}

pub struct VulkanParams {
    vk_params: pl_vulkan_params,
    c_ext: Data,
    c_opt: Data,
    c_device_name: CString,
}

impl Default for VulkanParams {
    fn default() -> Self {
        init_data!(c_ext, c_opt);
        VulkanParams {
            vk_params: unsafe { pl_vulkan_default_params },
            c_ext,
            c_opt,
            c_device_name: CString::new("").unwrap(),
        }
    }
}

set_params!(
    VulkanParams,
    vk_params,
    (
        instance,
        instance_params,
        surface,
        device,
        allow_software,
        async_transfer,
        async_compute,
        queue_count,
    ),
    (
        usize,
        &VulkanInstanceParams,
        u64,
        u64,
        bool,
        bool,
        bool,
        usize,
    ),
    (
        instance as VkInstance,
        &instance_params.vk_inst_params,
        surface as VkSurfaceKHR,
        device as VkPhysicalDevice,
        allow_software as bool,
        async_transfer as bool,
        async_compute as bool,
        queue_count as i32
    )
);

impl VulkanParams {
    pub fn set_device_name(&mut self, dev_name: &str) {
        self.c_device_name = CString::new(dev_name).unwrap();
        self.vk_params.device_name = self.c_device_name.as_ptr();
    }

    extensions!(vk_params);
}

pub struct Vulkan {
    vk: *const pl_vulkan,
}

impl Vulkan {
    pub fn new(ctx: &Context, params: &VulkanParams) -> Self {
        let vk =
            unsafe { pl_vulkan_create(ctx.get_mut_ptr(), &params.vk_params) };
        assert!(!vk.is_null());

        Vulkan { vk }
    }

    pub fn gpu(&self) -> Gpu {
        Gpu::new(self)
    }

    pub(crate) fn get_ptr(&self) -> *const pl_vulkan {
        self.vk
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            pl_vulkan_destroy(&mut self.vk);
        }
    }
}
