extern crate libplacebo;
extern crate sdl2;
extern crate structopt;

use libplacebo::colorspace::*;
use libplacebo::common::*;
use libplacebo::context::*;
use libplacebo::filter::*;
use libplacebo::gpu::*;
use libplacebo::renderer::*;
use libplacebo::swapchain::*;
use libplacebo::upload::*;
use libplacebo::vulkan::*;

use sdl2::event::Event;
use sdl2::image::ImageRWops;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use structopt::StructOpt;

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;

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

fn init_placebo() -> Context {
    let context_params =
        ContextParams::new(LogFunction::LogColor, LogLevel::LOG_DEBUG);
    Context::new(&context_params)
}

fn init_vulkan(
    window: &mut Window,
    ctx: &mut Context,
) -> (VulkanInstance, Vulkan, Swapchain) {
    let vk_extensions = window.vulkan_instance_extensions().unwrap();
    let num = vk_extensions.len();

    if num > 0 {
        println!("Requesting {} additional vulkan extensions", num);
        for ext in &vk_extensions {
            println!("{}", ext);
        }
    }

    let mut vk_inst_params: VulkanInstanceParams = Default::default();
    vk_inst_params.set_extensions(&vk_extensions);
    vk_inst_params.set_debug(true);

    let vk_inst = VulkanInstance::new(ctx, &vk_inst_params);

    let surface_handle =
        window.vulkan_create_surface(vk_inst.instance()).unwrap();

    let mut vk_params: VulkanParams = Default::default();
    vk_params.set_instance(vk_inst.instance());
    vk_params.set_surface(surface_handle);
    vk_params.set_allow_software(true);

    let vk = Vulkan::new(ctx, &vk_params);

    let mut swapchain_params: SwapchainParams = Default::default();
    swapchain_params.set_surface(surface_handle);

    let swapchain = Swapchain::new(&vk, &swapchain_params);

    let (w, h) = swapchain.resize(WINDOW_WIDTH, WINDOW_HEIGHT);

    if w != WINDOW_WIDTH || h != WINDOW_HEIGHT {
        println!("Note: window dimensions differ (got {}x{})", w, h);
    }

    (vk_inst, vk, swapchain)
}

fn upload_plane(
    path: &PathBuf,
    vk: &mut Vulkan,
    img_tex: &mut Tex,
    plane: &mut Plane,
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

    let mut plane_data: PlaneData = Default::default();
    plane_data.set_type_(&FmtType::FMT_UNORM);
    plane_data.set_width(surf_image.width() as usize);
    plane_data.set_height(surf_image.height() as usize);
    plane_data.set_pixel_stride(
        surf_image.pixel_format_enum().byte_size_per_pixel(),
    );
    plane_data.set_row_stride(surf_image.pitch() as usize);
    plane_data.set_pixels(surf_image.without_lock().unwrap());

    let mask = surf_image.pixel_format_enum().into_masks().unwrap();
    let mut masks: [u64; 4] = [
        u64::from(mask.rmask),
        u64::from(mask.gmask),
        u64::from(mask.bmask),
        u64::from(mask.amask),
    ];

    plane_data.data_from_mask(&mut masks);

    let gpu = vk.gpu();
    plane_data.upload_plane(&gpu, plane, img_tex);

    Ok(())
}

#[inline]
fn create_image(img_plane: &Plane) -> Image {
    let w = img_plane.width();
    let h = img_plane.height();

    let mut image: Image = Default::default();
    let planes: [&Plane; 4] = [&img_plane; 4];
    image.set_num_planes(1);
    image.set_planes(&planes);
    image.set_width(w);
    image.set_height(h);

    let mut color_repr: ColorRepr = Default::default();
    color_repr.set_alpha(&AlphaMode::ALPHA_INDEPENDENT);
    image.set_repr(&color_repr);
    image
}

#[inline]
fn create_target(
    frame: &SwapchainFrame,
    icc_profile: &mut Vec<u8>,
) -> RenderTarget {
    let mut target: RenderTarget = Default::default();
    target.render_target_from_swapchain(frame);

    if !icc_profile.is_empty() {
        let mut profile: IccProfile = Default::default();
        profile.set_data(&icc_profile);
        target.set_profile(&profile);
    }

    target
}

#[inline]
fn set_osd(image: &Image, target: &mut RenderTarget, osd_plane: &Plane) {
    let w = osd_plane.width();
    let h = osd_plane.height();
    let rect = Rect2D::new(0, 0, w, h);
    let target_overlay = Overlay::new(
        &osd_plane,
        &rect,
        &OverlayMode::OVERLAY_NORMAL,
        &[0.0; 3],
        &image.repr(),
        &image.color(),
    );
    let overlays: Vec<Overlay> = vec![target_overlay];
    target.set_overlays(&overlays);
}

#[inline]
fn render(renderer: &Renderer, image: &Image, target: &RenderTarget) {
    let mut render_params: RenderParams = Default::default();
    render_params.set_upscaler(&FilterConfig::get_filter_config(
        &FilterConfigs::EwaLanczos,
    ));
    renderer.render_image(&image, &target, &render_params);
}

fn main() -> std::io::Result<()> {
    let args = CliArgs::from_args();

    let mut img_plane: Plane = Default::default();
    let mut osd_plane: Plane = Default::default();
    let mut osd = false;
    let mut icc_profile: Vec<u8> = Vec::new();

    let start = Instant::now();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
        .window("libplacebo demo", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let mut ctx = init_placebo();
    let (_vk_inst, mut vk, swapchain) = init_vulkan(&mut window, &mut ctx);

    let gpu = vk.gpu();
    let mut img_tex = Tex::default(&gpu);
    let mut osd_tex = Tex::default(&gpu);

    upload_plane(&args.image, &mut vk, &mut img_tex, &mut img_plane).unwrap();

    if let Some(path) = &args.overlay {
        upload_plane(&path, &mut vk, &mut osd_tex, &mut osd_plane).unwrap();
        osd = true;
    }

    if let Some(path) = &args.iccprofile {
        let mut f = File::open(path)?;
        f.read_to_end(&mut icc_profile)?;
    }

    let renderer = Renderer::new(&ctx, &vk.gpu());

    // Resize the window to match the content
    let w = img_plane.width();
    let h = img_plane.height();
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

        let mut frame: SwapchainFrame = Default::default();
        let ok = swapchain.start_frame(&mut frame);

        if !ok {
            sleep(Duration::from_millis(10));
            continue;
        }

        let image = create_image(&img_plane);
        let mut target = create_target(&frame, &mut icc_profile);

        if osd {
            set_osd(&image, &mut target, &osd_plane);
        }

        render(&renderer, &image, &target);

        swapchain.submit_frame();
        swapchain.swap_buffers();
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
    Ok(())
}
