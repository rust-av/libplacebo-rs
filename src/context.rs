use crate::*;
use libplacebo_sys::*;

use std::ffi::c_void;

create_enum!(
    LogLevel,
    pl_log_level,
    (LOG_NONE, LOG_FATAL, LOG_ERR, LOG_WARN, LOG_INFO, LOG_DEBUG, LOG_TRACE)
);

#[derive(Clone, Copy, Debug)]
pub enum LogFunction {
    LogColor,
    LogSimple,
    NoLog,
}

pub struct ContextParams {
    ctx_params: pl_context_params,
}

impl ContextParams {
    pub fn new(log_cb: &LogFunction, log_level: &LogLevel) -> Self {
        type LogFunc =
            unsafe extern "C" fn(*mut c_void, pl_log_level, *const i8);
        let log_f: Option<LogFunc> = match log_cb {
            LogFunction::LogColor => Some(pl_log_color),
            LogFunction::LogSimple => Some(pl_log_simple),
            LogFunction::NoLog => None,
        };
        let ctx_params = pl_context_params {
            log_cb: log_f,
            log_level: LogLevel::to_pl_log_level(log_level),
            log_priv: 0 as *mut c_void,
        };

        ContextParams { ctx_params }
    }
}

pub struct Context {
    ctx: *mut pl_context,
}

impl Default for Context {
    fn default() -> Self {
        let ctx = unsafe {
            pl_context_create(PL_API_VER as i32, &pl_context_default_params)
        };
        assert!(!ctx.is_null());

        Context { ctx }
    }
}

impl Context {
    pub fn new(params: &ContextParams) -> Self {
        let ctx = unsafe {
            pl_context_create(PL_API_VER as i32, &params.ctx_params)
        };
        assert!(!ctx.is_null());

        Context { ctx }
    }

    pub fn update(&mut self, ctx_params: Option<&ContextParams>) {
        let mut par = unsafe { &pl_context_default_params };
        if let Some(v) = ctx_params {
            par = &v.ctx_params;
        }
        unsafe {
            pl_context_update(self.ctx, par);
        }
    }

    pub(crate) fn get_mut_ptr(&self) -> *mut pl_context {
        self.ctx
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            pl_context_destroy(&mut self.ctx);
        }
    }
}
