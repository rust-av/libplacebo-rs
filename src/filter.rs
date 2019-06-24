use crate::context::*;
use crate::*;

use libplacebo_sys::*;

//use std::ffi::c_void;

pub enum FilterFunctions {
    Box,
    Triangle,
    Hann,
    Hamming,
    Welch,
    Kaiser,
    BlackMan,
    Gaussian,
    Sinc,
    Jinc,
    Sphinx,
    Bcspline,
    CatmullRom,
    Mitchell,
    Robidoux,
    Robidouxsharp,
    Bicubic,
    Spline36,
    Spline64,
}

impl FilterFunctions {
    pub(crate) fn to_filter_function(&self) -> pl_filter_function {
        unsafe {
            let filter_func = match self {
                FilterFunctions::Box => pl_filter_function_box,
                FilterFunctions::Triangle => pl_filter_function_triangle,
                FilterFunctions::Hann => pl_filter_function_hann,
                FilterFunctions::Hamming => pl_filter_function_hamming,
                FilterFunctions::Welch => pl_filter_function_welch,
                FilterFunctions::Kaiser => pl_filter_function_kaiser,
                FilterFunctions::BlackMan => pl_filter_function_blackman,
                FilterFunctions::Gaussian => pl_filter_function_gaussian,
                FilterFunctions::Sinc => pl_filter_function_sinc,
                FilterFunctions::Jinc => pl_filter_function_jinc,
                FilterFunctions::Sphinx => pl_filter_function_sphinx,
                FilterFunctions::Bcspline => pl_filter_function_bcspline,
                FilterFunctions::CatmullRom => pl_filter_function_catmull_rom,
                FilterFunctions::Mitchell => pl_filter_function_mitchell,
                FilterFunctions::Robidoux => pl_filter_function_robidoux,
                FilterFunctions::Robidouxsharp => {
                    pl_filter_function_robidouxsharp
                }
                FilterFunctions::Bicubic => pl_filter_function_bicubic,
                FilterFunctions::Spline36 => pl_filter_function_spline36,
                FilterFunctions::Spline64 => pl_filter_function_spline64,
            };
            filter_func
        }
    }
}

set_struct!(FilterFunction, filter_func, pl_filter_function);

impl Default for FilterFunction {
    fn default() -> Self {
        let filter_func = pl_filter_function {
            resizable: false,
            tunable: [false; 2],
            weight: None,
            radius: 0.0,
            params: [0.0; 2],
        };
        FilterFunction { filter_func }
    }
}

impl FilterFunction {
    pub fn new(
        resizable: bool,
        tunable: &[bool; 2],
        radius: f32,
        params: &[f32; 2],
    ) -> Self {
        let filter_func = pl_filter_function {
            resizable: resizable,
            tunable: *tunable,
            weight: None,
            radius: radius,
            params: *params,
        };

        FilterFunction { filter_func }
    }

    //FIXME This function is a mess and it should be fixed and tested ASAP
    /*pub fn weight<F>(&mut self, _x: f64, _f: &F)
    where
        F: Fn(f64) -> f64,
    {
        struct PassData {
            data: Box<PassTrait>,
        }

        trait PassTrait {
            fn get(self: Box<Self>, v: f64) -> f64;
        }

        impl<F> PassTrait for F
        where
            F: Fn(f64) -> f64,
        {
            fn get(self: Box<Self>, v: f64) -> f64 {
                (self)(v)
            }
        }

        unsafe extern "C" fn inside(_k: *const pl_filter_function, x: f64) -> f64 {
            let mut a = x;
            let data = &mut a as *mut f64 as *mut c_void;
            let pass = Box::from_raw(data as *mut PassData);
            pass.data.get(x)
        }
        unsafe { inside(&pl_filter_function_sinc, 5.0); }
        self.filter_func.weight = Some(inside);
    }*/

    pub fn filter_function_eq(&self, b: &FilterFunction) -> bool {
        unsafe { pl_filter_function_eq(&self.filter_func, &b.filter_func) }
    }
}

// TODO name_filtered

pub enum FilterConfigs {
    Spline16,
    Spline36,
    Spline64,
    Box,
    Triangle,
    Gaussian,
    Sinc,
    Lanczos,
    Ginseng,
    EwaJinc,
    EwaLanczos,
    EwaGinseng,
    EwaHann,
    Haasnsoft,
    Bicubic,
    CatmullRom,
    Mitchell,
    Robidoux,
    Robidouxsharp,
    EwaRobidoux,
    EwaRobidouxsharp,
}

create_complete_struct!(
    FilterConfig,
    filter_config,
    pl_filter_config,
    (kernel, window, clamp, blur, taper, polar),
    (&FilterFunction, &FilterFunction, f32, f32, f32, bool),
    (
        0 as *const pl_filter_function,
        0 as *const pl_filter_function,
        0.0,
        0.0,
        0.0,
        false,
    ),
    (
        &kernel.filter_func,
        &window.filter_func,
        clamp as f32,
        blur as f32,
        taper as f32,
        polar as bool,
    )
);

get_ptr!(FilterConfig, filter_config, pl_filter_config);

impl FilterConfig {
    pub fn get_filter_config(filter: &FilterConfigs) -> Self {
        let filter_config = unsafe {
            match filter {
                FilterConfigs::Spline16 => pl_filter_spline16,
                FilterConfigs::Spline36 => pl_filter_spline36,
                FilterConfigs::Spline64 => pl_filter_spline64,
                FilterConfigs::Box => pl_filter_box,
                FilterConfigs::Triangle => pl_filter_triangle,
                FilterConfigs::Gaussian => pl_filter_gaussian,
                FilterConfigs::Sinc => pl_filter_sinc,
                FilterConfigs::Lanczos => pl_filter_lanczos,
                FilterConfigs::Ginseng => pl_filter_ginseng,
                FilterConfigs::EwaJinc => pl_filter_ewa_jinc,
                FilterConfigs::EwaLanczos => pl_filter_ewa_lanczos,
                FilterConfigs::EwaGinseng => pl_filter_ewa_ginseng,
                FilterConfigs::EwaHann => pl_filter_ewa_hann,
                FilterConfigs::Haasnsoft => pl_filter_haasnsoft,
                FilterConfigs::Bicubic => pl_filter_bicubic,
                FilterConfigs::CatmullRom => pl_filter_catmull_rom,
                FilterConfigs::Mitchell => pl_filter_mitchell,
                FilterConfigs::Robidoux => pl_filter_robidoux,
                FilterConfigs::Robidouxsharp => pl_filter_robidouxsharp,
                FilterConfigs::EwaRobidoux => pl_filter_ewa_robidoux,
                FilterConfigs::EwaRobidouxsharp => pl_filter_ewa_robidouxsharp,
            }
        };
        FilterConfig { filter_config }
    }

    pub fn filter_config_eq(&self, b: &FilterConfig) -> bool {
        unsafe { pl_filter_config_eq(&self.filter_config, &b.filter_config) }
    }

    pub fn filter_sample(&self, x: f64) -> f64 {
        unsafe { pl_filter_sample(&self.filter_config, x) }
    }
}

// TODO named_filter_config

create_struct!(
    FilterParams,
    filter_params,
    pl_filter_params,
    (
        config,
        lut_entries,
        filter_scale,
        cutoff,
        max_row_size,
        row_stride_align,
    ),
    (&FilterConfig, usize, f32, f32, usize, usize),
    (
        config.filter_config,
        lut_entries as i32,
        filter_scale as f32,
        cutoff as f32,
        max_row_size as i32,
        row_stride_align as i32,
    )
);

pub struct Filter {
    filter: *const pl_filter,
}

impl Filter {
    pub fn new(ctx: &mut Context, params: &FilterParams) -> Self {
        let filter = unsafe {
            pl_filter_generate(ctx.get_mut_ptr(), &params.filter_params)
        };
        assert!(!filter.is_null());
        Filter { filter }
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        unsafe {
            pl_filter_free(&mut self.filter);
        }
    }
}
