#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/placebo.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    #[test]
    fn new_context() {
        unsafe {
            let params = pl_context_params {
                log_cb: Some(pl_log_color),
                log_level: pl_log_level::PL_LOG_DEBUG,
                log_priv: ptr::null_mut(),
            };
            let mut ctx = pl_context_create(PL_API_VER as i32, &params);

            assert!(!ctx.is_null());

            pl_context_destroy(&mut ctx);

            assert!(ctx.is_null());
        }
    }
}
