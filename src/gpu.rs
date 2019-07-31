use crate::vulkan::*;
use crate::*;

use libplacebo_sys::*;

use std::default::Default;
use std::ptr::{null, null_mut};

create_enum!(
    FmtType,
    pl_fmt_type,
    (
        FMT_UNKNOWN,
        FMT_UNORM,
        FMT_SNORM,
        FMT_UINT,
        FMT_SINT,
        FMT_FLOAT,
        FMT_TYPE_COUNT,
    )
);

simple_enum!(
    BufType,
    (
        BUF_INVALID,
        BUF_TEX_TRANSFER,
        BUF_UNIFORM,
        BUF_STORAGE,
        BUF_TEXEL_UNIFORM,
        BUF_TEXEL_STORAGE,
        BUF_PRIVATE,
        BUF_TYPE_COUNT,
    )
);

simple_enum!(
    HandleType,
    (HANDLE_FD, HANDLE_WIN32, HANDLE_WIN32_KMT, HANDLE_DMA_BUF)
);

simple_enum!(BufMemType, (BUF_MEM_AUTO, BUF_MEM_HOST, BUF_MEM_DEVICE));

pub union Handle {
    handle: pl_handle,
}

default_struct!(Handle, handle, pl_handle, (handle), (null_mut()));

set_struct!(SharedMem, shared_mem, pl_shared_mem);

impl Default for SharedMem {
    fn default() -> Self {
        let handle: Handle = Default::default();
        let shared_mem = pl_shared_mem {
            handle: unsafe { handle.handle },
            size: 0 as usize,
            offset: 0 as usize,
        };

        SharedMem { shared_mem }
    }
}

#[derive(Clone)]
pub struct Gpu {
    gpu: *const pl_gpu,
}

impl Gpu {
    pub fn new(vk: &Vulkan) -> Self {
        let gpu = unsafe { (*vk.get_ptr()).gpu };
        Gpu { gpu }
    }

    pub fn gpu_flush(&self) {
        unsafe {
            pl_gpu_flush(self.gpu);
        }
    }

    pub fn gpu_finish(&self) {
        unsafe {
            pl_gpu_finish(self.gpu);
        }
    }

    pub fn pass_run(&self, params: &pl_pass_run_params) {
        unsafe {
            pl_pass_run(self.gpu, params);
        }
    }

    pub(crate) fn get_ptr(&self) -> *const pl_gpu {
        self.gpu
    }
}

set_struct!(BufParams, buf_params, pl_buf_params);

impl Default for BufParams {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        let handle_type = pl_handle_type::PL_HANDLE_WIN32;
        #[cfg(target_os = "linux")]
        let handle_type = pl_handle_type::PL_HANDLE_FD;

        let buf_params = pl_buf_params {
            type_: pl_buf_type::PL_BUF_INVALID,
            size: 0,
            host_mapped: false,
            host_writable: false,
            host_readable: false,
            memory_type: pl_buf_mem_type::PL_BUF_MEM_AUTO,
            format: null(),
            handle_type,
            initial_data: null(),
            user_data: null_mut(),
        };
        BufParams { buf_params }
    }
}

pub struct Buf {
    buf: *const pl_buf,
    gpu: *const pl_gpu,
}

impl Buf {
    pub fn new(gpu: &Gpu, params: &BufParams) -> Self {
        let buf = unsafe { pl_buf_create(gpu.gpu, &params.buf_params) };
        assert!(!buf.is_null());
        Buf { buf, gpu: gpu.gpu }
    }

    pub(crate) fn get_ptr(&self) -> *const pl_buf {
        self.buf
    }
}

impl Drop for Buf {
    fn drop(&mut self) {
        unsafe {
            pl_buf_destroy(self.gpu, &mut self.buf);
        }
    }
}

set_struct!(TexParams, tex_params, pl_tex_params);
impl Default for TexParams {
    fn default() -> Self {
        let shared_mem: SharedMem = Default::default();
        #[cfg(target_os = "windows")]
        let import_handle = pl_handle_type::PL_HANDLE_WIN32;
        #[cfg(target_os = "windows")]
        let export_handle = pl_handle_type::PL_HANDLE_WIN32;
        #[cfg(target_os = "linux")]
        let import_handle = pl_handle_type::PL_HANDLE_FD;
        #[cfg(target_os = "linux")]
        let export_handle = pl_handle_type::PL_HANDLE_FD;

        let tex_params = pl_tex_params {
            w: 0,
            h: 0,
            d: 0,
            format: null(),
            sampleable: false,
            renderable: false,
            storable: false,
            blit_src: false,
            blit_dst: false,
            host_writable: false,
            host_readable: false,
            sample_mode: pl_tex_sample_mode::PL_TEX_SAMPLE_NEAREST,
            address_mode: pl_tex_address_mode::PL_TEX_ADDRESS_CLAMP,
            export_handle,
            import_handle,
            shared_mem: shared_mem.shared_mem,
            initial_data: null(),
            user_data: null_mut(),
        };
        TexParams { tex_params }
    }
}

pub struct Tex {
    tex: *const pl_tex,
    gpu: *const pl_gpu,
}

impl Tex {
    pub fn default(gpu: &Gpu) -> Self {
        Tex {
            tex: null(),
            gpu: gpu.gpu,
        }
    }

    pub fn new(gpu: &Gpu, params: &TexParams) -> Self {
        let tex = unsafe { pl_tex_create(gpu.gpu, &params.tex_params) };
        assert!(!tex.is_null());

        Tex { tex, gpu: gpu.gpu }
    }

    /*
     TODO We need to test these ones because surely they hide some memory errors

     pub fn tex_recreate(&mut self, params: &TexParams) -> bool {
        unsafe { pl_tex_recreate(self.gpu.gpu, &mut self.tex, &params.tex_params) }
    }*/

    /*pub fn tex_invalidate(&mut self) {
        unsafe { pl_tex_invalidate(self.gpu.gpu, self.tex) };
    }*/

    /*pub fn tex_clear(&mut self, color: &f32) {
        unsafe { pl_tex_clear(self.gpu.gpu, self.tex, color) };
    }*/

    /*pub fn tex_blit(&self, dst: &Tex, src: &Tex, dst_src: &Rect3D, src_rc: &Rect3D) {
        unsafe {
            pl_tex_blit(
                self.gpu.gpu,
                dst.tex,
                src.tex,
                dst_src.internal_object(),
                src_rc.internal_object(),
            )
        };
    }*/

    /*
    pub fn tex_upload(&self, params: &pl_tex_transfer_params) -> bool {
        unsafe { pl_tex_upload(self.gpu.gpu, params) as bool }
    }

    pub fn tex_download(&self, params: &pl_tex_transfer_params) -> bool {
        unsafe { pl_tex_download(self.gpu.gpu, params) as bool }
    }

    pub fn tex_export(&self, sync: &Sync) -> bool {
        unsafe { pl_tex_export(self.gpu.gpu, self.tex, sync.sync) }
    } */

    pub(crate) fn get_ptr(&self) -> *const pl_tex {
        self.tex
    }

    pub(crate) fn set_ptr(&mut self, ptr: *const pl_tex) {
        self.tex = ptr;
    }
}

impl Drop for Tex {
    fn drop(&mut self) {
        unsafe {
            pl_tex_destroy(self.gpu, &mut self.tex);
        }
    }
}
