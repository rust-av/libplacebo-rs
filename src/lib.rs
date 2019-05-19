use libplacebo_sys::*;

pub struct Context {
    ctx: *mut pl_context,
}

impl Default for Context {
    fn default() -> Self {
        let ctx = unsafe { pl_context_create(PL_API_VER as i32, &pl_context_default_params) };
        assert!(!ctx.is_null());

        Context { ctx }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            let ctx = &mut self.ctx;
            pl_context_destroy(ctx);
        }
    }
}

pub mod vulkan {
    use super::*;

    pub struct Instance {
        inst: *const pl_vk_inst,
    }

    impl Instance {
        pub fn new(ctx: Context) -> Self {
            let inst = unsafe { pl_vk_inst_create(ctx.ctx, &pl_vk_inst_default_params) };
            assert!(!inst.is_null());

            Instance { inst }
        }
    }

    impl Drop for Instance {
        fn drop(&mut self) {
            unsafe {
                let inst = &mut self.inst;
                pl_vk_inst_destroy(inst);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
