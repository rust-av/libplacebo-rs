use crate::gpu::*;
use crate::renderer::*;
use crate::*;

use libplacebo_sys::*;

use std::ffi::c_void;
use std::ptr::null;

set_struct!(PlaneData, plane_data, pl_plane_data);
default_struct!(
    PlaneData,
    plane_data,
    pl_plane_data,
    (
        type_,
        width,
        height,
        component_size,
        component_pad,
        component_map,
        pixel_stride,
        row_stride,
        pixels,
        buf,
        buf_offset,
    ),
    (
        pl_fmt_type::PL_FMT_UNKNOWN,
        0,
        0,
        [0; 4],
        [0; 4],
        [0; 4],
        0,
        0,
        null(),
        null(),
        0,
    )
);

set_params!(
    PlaneData,
    plane_data,
    (
        type_,
        width,
        height,
        component_size,
        component_pad,
        component_map,
        pixel_stride,
        row_stride,
        pixels,
        buf,
        buf_offset,
    ),
    (
        &FmtType,
        usize,
        usize,
        &[i32; 4],
        &[i32; 4],
        &[i32; 4],
        usize,
        usize,
        &[u8],
        &Buf,
        usize,
    ),
    (
        FmtType::to_pl_fmt_type(type_),
        width as i32,
        height as i32,
        *component_size as [i32; 4],
        *component_pad as [i32; 4],
        *component_map as [i32; 4],
        pixel_stride as usize,
        row_stride as usize,
        pixels.as_ptr() as *const c_void,
        buf.get_ptr(),
        buf_offset as usize,
    )
);

impl PlaneData {
    pub fn data_from_mask(&mut self, mask: &mut [u64; 4]) {
        unsafe {
            pl_plane_data_from_mask(&mut self.plane_data, mask.as_mut_ptr());
        }
    }

    pub fn upload_plane(
        &self,
        gpu: &Gpu,
        out_plane: &mut Plane,
        tex: &mut Tex,
    ) {
        let mut tex_i = null();
        let ok = unsafe {
            pl_upload_plane(
                gpu.get_ptr(),
                out_plane.get_mut_ptr(),
                &mut tex_i,
                &self.plane_data,
            )
        };
        tex.set_ptr(tex_i);
        assert!(ok);
    }
}
