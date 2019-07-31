extern crate libplacebo_sys;
extern crate sdl2;
extern crate structopt;

use libplacebo_sys::*;

use sdl2::event::Event;
use sdl2::image::ImageRWops;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::ffi::c_void;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process;
use std::ptr::{null, null_mut};
use std::thread::sleep;
use std::time::{Duration, Instant};
use structopt::StructOpt;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

macro_rules! null_plane_struct {
    ($var:ident) => {
        let $var = pl_plane {
            texture: null(),
            components: 0,
            component_mapping: [0; 4],
            shift_x: 0.0,
            shift_y: 0.0,
        };
    };
}

#[derive(Debug, StructOpt)]
struct CliArgs {
    /// The image to be rendered
    #[structopt(parse(from_os_str))]
    image: PathBuf,
    /// The overlay file to be applied
    #[structopt(parse(from_os_str))]
    overlay: Option<PathBuf>,
    /// The iccprofile associated to the image
    #[structopt(parse(from_os_str))]
    iccprofile: Option<PathBuf>,
}

struct Placebo {
    ctx: *mut pl_context,
    vk: *const pl_vulkan,
    vk_inst: *const pl_vk_inst,
    swapchain: *const pl_swapchain,
    img_tex: *const pl_tex,
    osd_tex: *const pl_tex,
    img_plane: pl_plane,
    osd_plane: pl_plane,
    renderer: *mut pl_renderer,
    icc_profile: Vec<u8>,
}

fn init_placebo(pl: &mut Placebo) {
    let context_params = pl_context_params {
        log_cb: Some(pl_log_color),
        log_level: pl_log_level::PL_LOG_DEBUG,
        log_priv: null_mut(),
    };
    let ctx = unsafe { pl_context_create(PL_API_VER as i32, &context_params) };
    assert!(!ctx.is_null());

    pl.ctx = ctx;
}

fn init_vulkan(window: &mut Window, pl: &mut Placebo) {
    let vk_extensions = window.vulkan_instance_extensions().unwrap();
    let num = vk_extensions.len();

    let c_str: Vec<_> = vk_extensions.iter().map(|arg| arg.as_ptr()).collect();

    let mut vk_inst_params = unsafe { pl_vk_inst_default_params };

    vk_inst_params.extensions = c_str.as_ptr() as *mut *const i8;
    vk_inst_params.num_extensions = num as i32;
    vk_inst_params.debug = true;

    if num > 0 {
        println!("Requesting {} additional vulkan extensions", num);
        for ext in &vk_extensions {
            println!("{}", ext);
        }
    }

    let vk_inst = unsafe { pl_vk_inst_create(pl.ctx, &vk_inst_params) };
    assert!(!vk_inst.is_null());
    pl.vk_inst = vk_inst;

    let inst = unsafe { (*pl.vk_inst).instance as usize };
    let surface_handle = window.vulkan_create_surface(inst).unwrap();

    let mut vk_params = unsafe { pl_vulkan_default_params };
    vk_params.instance = unsafe { (*pl.vk_inst).instance };
    vk_params.surface = surface_handle as VkSurfaceKHR;
    vk_params.allow_software = true;

    let vk = unsafe { pl_vulkan_create(pl.ctx, &vk_params) };
    assert!(!vk.is_null());
    pl.vk = vk;

    let swapchain_params = pl_vulkan_swapchain_params {
        surface: surface_handle as VkSurfaceKHR,
        present_mode: VkPresentModeKHR::VK_PRESENT_MODE_IMMEDIATE_KHR,
        surface_format: VkSurfaceFormatKHR {
            format: VkFormat::VK_FORMAT_UNDEFINED,
            colorSpace: VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        },
        swapchain_depth: 3,
    };

    let swapchain =
        unsafe { pl_vulkan_create_swapchain(pl.vk, &swapchain_params) };
    assert!(!swapchain.is_null());

    pl.swapchain = swapchain;

    let mut w = WINDOW_WIDTH as i32;
    let mut h = WINDOW_HEIGHT as i32;

    let ok = unsafe { pl_swapchain_resize(pl.swapchain, &mut w, &mut h) };
    if !ok {
        eprintln!("Failed resizing vulkan swapchain!");
        process::exit(2);
    }

    if w != WINDOW_WIDTH as i32 || h != WINDOW_HEIGHT as i32 {
        println!("Note: window dimensions differ (got {}x{})", w, h);
    }
}

fn upload_plane(
    path: &PathBuf,
    pl: &mut Placebo,
    is_image: bool,
) -> std::io::Result<()> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let rw = RWops::from_bytes(&buf).unwrap();
    let mut surf_image = rw.load().unwrap();

    match surf_image.pixel_format_enum() {
        PixelFormatEnum::Index1LSB
        | PixelFormatEnum::Index1MSB
        | PixelFormatEnum::Index4LSB
        | PixelFormatEnum::Index4MSB => {
            let mut fix_surf = Surface::new(
                surf_image.width(),
                surf_image.height(),
                PixelFormatEnum::ABGR8888,
            )
            .unwrap();
            surf_image.blit(None, &mut fix_surf, None).unwrap();
            surf_image = fix_surf;
        }
        _ => {}
    }

    let mut plane_data = pl_plane_data {
        type_: pl_fmt_type::PL_FMT_UNORM,
        width: surf_image.width() as i32,
        height: surf_image.height() as i32,
        component_size: [0; 4],
        component_pad: [0; 4],
        component_map: [0; 4],
        pixel_stride: surf_image.pixel_format_enum().byte_size_per_pixel(),
        row_stride: surf_image.pitch() as usize,
        pixels: surf_image.without_lock().unwrap().as_ptr() as *const c_void,
        buf: null(),
        buf_offset: 0,
    };

    let mask = surf_image.pixel_format_enum().into_masks().unwrap();
    let mut masks: [u64; 4] = [
        u64::from(mask.rmask),
        u64::from(mask.gmask),
        u64::from(mask.bmask),
        u64::from(mask.amask),
    ];
    unsafe { pl_plane_data_from_mask(&mut plane_data, masks.as_mut_ptr()) }

    let vk_gpu = unsafe { (*pl.vk).gpu };
    let mut ok = is_image;
    if ok {
        ok = unsafe {
            pl_upload_plane(
                vk_gpu,
                &mut pl.img_plane,
                &mut pl.img_tex,
                &plane_data,
            )
        }
    } else {
        ok = unsafe {
            pl_upload_plane(
                vk_gpu,
                &mut pl.osd_plane,
                &mut pl.osd_tex,
                &plane_data,
            )
        }
    }

    if !ok {
        eprintln!("Failed uploading plane!");
        process::exit(2);
    }

    Ok(())
}

fn init_rendering(args: &CliArgs, pl: &mut Placebo) -> std::io::Result<()> {
    upload_plane(&args.image, pl, true).unwrap();

    if let Some(path) = &args.overlay {
        upload_plane(&path, pl, false).unwrap();
    }

    if let Some(path) = &args.iccprofile {
        let mut f = File::open(path)?;
        f.read_to_end(&mut pl.icc_profile)?;
    }

    let vk_gpu = unsafe { (*pl.vk).gpu };
    let renderer = unsafe { pl_renderer_create(pl.ctx, vk_gpu) };
    assert!(!renderer.is_null());
    pl.renderer = renderer;

    Ok(())
}

fn render_frame(pl: &mut Placebo, frame: &mut pl_swapchain_frame) {
    let img = pl.img_plane.texture;
    let w = unsafe { (*img).params.w };
    let h = unsafe { (*img).params.h };
    let icc_profile = pl_icc_profile {
        data: null(),
        len: 0,
        signature: 0,
    };

    let mut image = pl_image {
        signature: 0,
        num_planes: 1,
        planes: [pl.img_plane; 4],
        repr: unsafe { pl_color_repr_unknown },
        color: unsafe { pl_color_space_unknown },
        profile: icc_profile,
        width: w,
        height: h,
        src_rect: pl_rect2df {
            x0: 0.0,
            y0: 0.0,
            x1: 0.0,
            y1: 0.0,
        },
        overlays: null(),
        num_overlays: 0,
    };

    image.repr.alpha = pl_alpha_mode::PL_ALPHA_INDEPENDENT;

    let mut render_params = unsafe { pl_render_default_params };
    render_params.upscaler = unsafe { &pl_filter_ewa_lanczos };

    let mut target = pl_render_target {
        fbo: null(),
        dst_rect: pl_rect2d {
            x0: 0,
            y0: 0,
            x1: 0,
            y1: 0,
        },
        repr: unsafe { pl_color_repr_unknown },
        color: unsafe { pl_color_space_unknown },
        profile: icc_profile,
        overlays: null(),
        num_overlays: 0,
    };

    unsafe {
        pl_render_target_from_swapchain(&mut target, frame);
    }

    if !pl.icc_profile.is_empty() {
        let icc_profile = pl_icc_profile {
            data: pl.icc_profile.as_ptr() as *const c_void,
            len: pl.icc_profile.len(),
            signature: 0,
        };
        target.profile = icc_profile;
    }

    let osd = pl.osd_plane.texture;
    if !osd.is_null() {
        let w = unsafe { (*osd).params.w };
        let h = unsafe { (*osd).params.h };
        let target_ol = pl_overlay {
            plane: pl.osd_plane,
            rect: pl_rect2d {
                x0: 0,
                y0: 0,
                x1: w,
                y1: h,
            },
            mode: pl_overlay_mode::PL_OVERLAY_NORMAL,
            base_color: [0.0; 3],
            repr: image.repr,
            color: image.color,
        };
        target.overlays = &target_ol;
        target.num_overlays = 1;
    }

    let ok = unsafe {
        pl_render_image(pl.renderer, &image, &target, &render_params)
    };
    if !ok {
        eprintln!("Failed rendering frame!");
        process::exit(2);
    }
}

fn uninit_placebo(pl: &mut Placebo) {
    unsafe {
        pl_renderer_destroy(&mut pl.renderer);
        pl_tex_destroy((*pl.vk).gpu, &mut pl.img_tex);
        pl_tex_destroy((*pl.vk).gpu, &mut pl.osd_tex);
        pl_swapchain_destroy(&mut pl.swapchain);
        pl_vulkan_destroy(&mut pl.vk);
        pl_vk_inst_destroy(&mut pl.vk_inst);
        pl_context_destroy(&mut pl.ctx);
    }
}

fn main() {
    let args = CliArgs::from_args();

    let start = Instant::now();

    null_plane_struct!(img_plane);
    null_plane_struct!(osd_plane);

    let mut pl = Placebo {
        ctx: null_mut(),
        vk: null(),
        vk_inst: null(),
        swapchain: null(),
        img_tex: null(),
        osd_tex: null(),
        renderer: null_mut(),
        img_plane,
        osd_plane,
        icc_profile: Vec::new(),
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
        .window("libplacebo demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    init_placebo(&mut pl);
    init_vulkan(&mut window, &mut pl);
    init_rendering(&args, &mut pl).unwrap();

    // Resize the window to match the content
    let img = pl.img_plane.texture;
    let w = unsafe { (*img).params.w };
    let h = unsafe { (*img).params.h };
    window.set_size(w as u32, h as u32).unwrap();

    let mut last = Instant::now();
    let mut frames = 0;
    println!("Took {} ms for initialization", (last - start).as_millis());

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let color_repr = unsafe { pl_color_repr_unknown };
        let color_space = unsafe { pl_color_space_unknown };

        let mut frame = pl_swapchain_frame {
            fbo: null(),
            flipped: false,
            color_repr,
            color_space,
        };

        let ok = unsafe { pl_swapchain_start_frame(pl.swapchain, &mut frame) };
        if !ok {
            sleep(Duration::from_millis(10));
            continue;
        }

        render_frame(&mut pl, &mut frame);
        let ok = unsafe { pl_swapchain_submit_frame(pl.swapchain) };
        if !ok {
            eprintln!("Failed submitting frame!");
            process::exit(3);
        }

        unsafe { pl_swapchain_swap_buffers(pl.swapchain) };
        frames += 1;

        let now = Instant::now();
        let millis = (now - last).as_millis();
        if millis > 5000 {
            println!(
                "{} frames in {} ms = {} FPS",
                frames,
                millis,
                1000.0 * f64::from(frames) / millis as f64
            );
            last = now;
            frames = 0;
        }
    }
    uninit_placebo(&mut pl);
}
