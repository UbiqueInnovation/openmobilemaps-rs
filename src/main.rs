use autocxx::subclass::prelude::*;
use ffi::LoaderInterfaceImpl_methods;
use glium::{
    uniform,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction, Uniforms},
};
use glutin::{
    dpi::{PhysicalSize, Size},
    ContextBuilder, GlRequest,
};
use image::GenericImageView;
// use opengl_graphics::{TextureSettings, Wrap};
// use sdl2::{
//     event::{DisplayEvent, WindowEvent},
//     mouse::{MouseState, MouseWheelDirection},
//     video::GLProfile,
// };
// use glium::glutin;
use std::{
    default::Default,
    ffi::{CStr, CString},
    pin::Pin,
    sync::{mpsc::Sender, Arc, Mutex},
    time::Duration,
};
use tokio::runtime::Runtime;

pub(crate) unsafe fn cxx_const_cast<T: UniquePtrTarget>(value: &T) -> Pin<&mut T> {
    #![inline]
    //! Presents an immutable reference as a mutable one for the purpose of calling a CXX bridge
    //! function (casts the constness away). The mutable reference must not actually be mutated!
    //! (Otherwise, bring mutability into the Rust code.)
    //!
    //! This is meant as a last resort to avoid having to write a C++ wrapper function every
    //! time some API function isn't declared as `const` on the C++ side, even though it should
    //! be. In that wrapper, the same thing would be done with a C++ `const_cast<...>(...)`
    //! anyway.

    #[allow(clippy::cast_ref_to_mut)]
    Pin::new_unchecked(&mut *(value as *const T as *mut T))
}

pub(crate) unsafe fn cxx_shared_cast<T: SharedPtrTarget>(value: &T) -> Pin<&mut T> {
    #![inline]
    //! Presents an immutable reference as a mutable one for the purpose of calling a CXX bridge
    //! function (casts the constness away). The mutable reference must not actually be mutated!
    //! (Otherwise, bring mutability into the Rust code.)
    //!
    //! This is meant as a last resort to avoid having to write a C++ wrapper function every
    //! time some API function isn't declared as `const` on the C++ side, even though it should
    //! be. In that wrapper, the same thing would be done with a C++ `const_cast<...>(...)`
    //! anyway.

    #[allow(clippy::cast_ref_to_mut)]
    Pin::new_unchecked(&mut *(value as *const T as *mut T))
}

#[extern_rust_function]
pub fn log_rs(log_statement: String) {
    // print!("{log_statement}");
}

fn main() {
    // let sdl = sdl2::init().unwrap();
    // let video_subsystem = sdl.video().unwrap();

    // let gl_attr = video_subsystem.gl_attr();
    // gl_attr.set_context_profile(GLProfile::Core);
    // gl_attr.set_context_version(4, 1);

    // gl_attr.set_depth_size(24);
    // gl_attr.set_red_size(8);
    // gl_attr.set_green_size(8);
    // gl_attr.set_blue_size(8);
    // gl_attr.set_alpha_size(8);
    // gl_attr.set_stencil_size(8);
    // // gl_attr.set_framebuffer_srgb_compatible(true);
    // // gl_attr.set_accelerated_visual(true);

    // // gl_attr.set_alpha_size(3);
    // // gl_attr.set_share_with_current_context(true);

    // let window = video_subsystem
    //     .window("Game", 1200, 900)
    //     .opengl()
    //     .resizable()
    //     .build()
    //     .unwrap();

    // let gl_context = window.gl_create_context().unwrap();

    // window.gl_make_current(&gl_context).unwrap();

    // unsafe {
    //     gl::Enable(gl::DEPTH_TEST);
    //     gl::DepthFunc(gl::LESS);
    // }

    // let shader_version =
    //     unsafe { CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *mut i8) };
    // println!("{}", shader_version.to_string_lossy());

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(Size::Physical(PhysicalSize {
            width: 1200,
            height: 900,
        }))
        .with_visible(false);
    let cb = ContextBuilder::new()
        .with_double_buffer(Some(true))
        .with_multisampling(32)
        .with_depth_buffer(8)
        .with_stencil_buffer(8)
        .with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    gl::load_with(|s| display.gl_window().get_proc_address(s) as *const std::os::raw::c_void);
    use glium::Surface;
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.finish();
    use glium::backend::Facade;

    let mut arrays = 0;
    unsafe { gl::GenVertexArrays(10, &mut arrays) };
    unsafe { gl::BindVertexArray(arrays) };

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let coordsystem = ffi::CoordinateSystemFactory::getEpsg3857System();
    let map_config = ffi::MapConfig::new(coordsystem.within_unique_ptr()).within_unique_ptr();

    let (tx, rx) = std::sync::mpsc::channel();
    SchedulerInterfaceImplPool::STATIC_RUNTIME_POOL
        .lock()
        .unwrap()
        .0 = Some(tx);
    // let scheduler = SchedulerInterfaceImpl::new_cpp_owned(scheduler);
    // let scheduler = SchedulerInterfaceImpl::as_SchedulerInterface_unique_ptr(scheduler);
    // let shared_ptr = ffi::transform_unique(scheduler);
    let scheduler = ffi::SchedulerInterfaceStaticWrapper::new().within_unique_ptr();
    let scheduler = ffi::transform_unique(scheduler);
    // println!("{}", shared_ptr.is_null());
    let mut map_interface: UniquePtr<ffi::MapInterface> =
        ffi::MapInterface::createWithOpenGl(&map_config, &scheduler, 1.0);
    let (invalidate_sender, invalidate_receiver) = std::sync::mpsc::channel();
    // println!("DPI: {}", video_subsystem.display_dpi(0).unwrap().0);
    let callbacks = MapCallbackInterfaceImpl {
        sender: Some(invalidate_sender),
        ..Default::default()
    };
    let callbacks = MapCallbackInterfaceImpl::new_cpp_owned(callbacks);
    let callbackInterface = MapCallbackInterfaceImpl::as_MapCallbackInterface_unique_ptr(callbacks);

    map_interface
        .pin_mut()
        .setCallbackHandler(&ffi::to_map_callback_interface_shared_pointer(
            callbackInterface,
        ));
    let (ready_state_sender, ready_state_receiver) = std::sync::mpsc::channel();
    let ready_state = MapReadyCallbackInterfaceImpl {
        sender: Some(ready_state_sender),
        ..Default::default()
    };
    let ready_state = MapReadyCallbackInterfaceImpl::new_cpp_owned(ready_state);
    let ready_state_interface =
        MapReadyCallbackInterfaceImpl::as_MapReadyCallbackInterface_unique_ptr(ready_state);

    // map_interface.pin_mut().setViewportSize(ffi::Vec2I:)

    let loader = LoaderInterfaceImplRs {
        display: Some(display.clone()),
        ..Default::default()
    };
    let loader = LoaderInterfaceImplRs::new_cpp_owned(loader);

    let loader = LoaderInterfaceImplRs::as_LoaderInterfaceImpl_unique_ptr(loader);

    println!("start downcast");
    let loader = ffi::LoaderInterfaceImpl::toShared(loader);

    let loader = ffi::LoaderInterfaceImpl::asLoaderInterface(loader);
    println!("downcast succeeded");

    println!("LoaderInterface: {}", loader.is_null());

    let mut builder = ffi::Tiled2dMapRasterLayerInterfaceBuilder::builder().within_unique_ptr();
    println!("add loader");
    builder.pin_mut().addLoader(loader);

    println!("added loader");
    let config_wrapper = ffi::Tiled2dMapLayerConfigWrapper::new().within_unique_ptr();
    // builder.pin_mut().addConfig();

    let config = ffi::Tiled2dMapLayerConfigWrapper::asTiled2dMapLayerConfig(config_wrapper);

    builder.pin_mut().setConfig(config);
    let mut tiled = builder.pin_mut().build();

    let polygon_layer = ffi::PolygonLayerInterface::create();
    let mut polygon_coord_builder = ffi::PolygonCoordBuilder::new().within_unique_ptr();
    polygon_coord_builder.pin_mut().addCoord(&ffi::Coord::new(ffi::CoordinateSystemIdentifiers::EPSG2056(), 2684200.0, 1244833.3, 0.0).within_unique_ptr());
     polygon_coord_builder.pin_mut().addCoord(&ffi::Coord::new(ffi::CoordinateSystemIdentifiers::EPSG2056(),  2684200.0, 1345833.3, 0.0).within_unique_ptr());
      polygon_coord_builder.pin_mut().addCoord(&ffi::Coord::new(ffi::CoordinateSystemIdentifiers::EPSG2056(), 2785200.0, 1345833.3, 0.0).within_unique_ptr());
       polygon_coord_builder.pin_mut().addCoord(&ffi::Coord::new(ffi::CoordinateSystemIdentifiers::EPSG2056(), 2785200.0, 1244833.3, 0.0).within_unique_ptr());

   
    let polygon_coord = polygon_coord_builder.pin_mut().build();
    let polygon_info = ffi::PolygonInfo::new(
        "test",
        polygon_coord,
        ffi::Color::new(1.0, 0.0, 0.0, 1.0).within_unique_ptr(),
        ffi::Color::new(1.0, 0.4, 0.4, 1.0).within_unique_ptr(),
    )
    .within_unique_ptr();
    let p = unsafe { cxx_shared_cast(polygon_layer.as_ref().unwrap()) };
    p.add(&polygon_info);


     let mut polygon_coord_builder = ffi::PolygonCoordBuilder::new().within_unique_ptr();
    polygon_coord_builder.pin_mut().addCoord(&ffi::Coord::new(ffi::CoordinateSystemIdentifiers::EPSG2056(), 2694200.0, 1254833.3, 0.0).within_unique_ptr());
     polygon_coord_builder.pin_mut().addCoord(&ffi::Coord::new(ffi::CoordinateSystemIdentifiers::EPSG2056(),  2694200.0, 1355833.3, 0.0).within_unique_ptr());
      polygon_coord_builder.pin_mut().addCoord(&ffi::Coord::new(ffi::CoordinateSystemIdentifiers::EPSG2056(), 2795200.0, 1355833.3, 0.0).within_unique_ptr());
       polygon_coord_builder.pin_mut().addCoord(&ffi::Coord::new(ffi::CoordinateSystemIdentifiers::EPSG2056(), 2795200.0, 1254833.3, 0.0).within_unique_ptr());

   
    let polygon_coord = polygon_coord_builder.pin_mut().build();
    let polygon_info = ffi::PolygonInfo::new(
        "test",
        polygon_coord,
        ffi::Color::new(0.0, 1.0, 0.0, 1.0).within_unique_ptr(),
        ffi::Color::new(1.0, 0.4, 0.4, 1.0).within_unique_ptr(),
    )
    .within_unique_ptr();
    let p = unsafe { cxx_shared_cast(polygon_layer.as_ref().unwrap()) };
    p.add(&polygon_info);


   let p = unsafe { cxx_shared_cast(polygon_layer.as_ref().unwrap()) };
    let the_layer = p.asLayerInterface();
    map_interface.pin_mut().addLayer(&the_layer);
  

    map_interface
        .pin_mut()
        .setViewportSize(&ffi::Vec2I::new(1200, 900).within_unique_ptr());

    println!("before cast as layerInterface");
    let blub = ffi::down_cast_to_layer_interface(tiled);
    println!("after cast as layerInterface");

    map_interface.pin_mut().addLayer(&blub);
    // let loader_list = ffi::get_loader_list(tiled);

    println!("layer added");

    let red_color = ffi::Color::new(1.0, 1.0, 1.0, 0.5).within_unique_ptr();
    map_interface.pin_mut().setBackgroundColor(&red_color);
    let camera = map_interface.pin_mut().getCamera();

    let c: Pin<&mut ffi::MapCamera2dInterface> =
        unsafe { cxx_const_cast(camera.as_ref().unwrap()) };

    c.setMaxZoom(0.0);
    let c: Pin<&mut ffi::MapCamera2dInterface> =
        unsafe { cxx_const_cast(camera.as_ref().unwrap()) };
    c.setMinZoom(f64::MAX);

    let centerCoord = ffi::Coord::new(
        ffi::CoordinateSystemIdentifiers::EPSG4326(),
        8.543912536386152,
        47.37623511643675,
        0.0,
    )
    .within_unique_ptr();

    let c: Pin<&mut ffi::MapCamera2dInterface> =
        unsafe { cxx_const_cast(camera.as_ref().unwrap()) };

    c.moveToCenterPositionZoom(&centerCoord, 50000.0, false);

    map_interface.pin_mut().resume();
    map_interface.pin_mut().drawFrame();

    let mut counter = 0;
    loop {
        let frame = display.draw();
        if let Ok(task) = rx.recv_timeout(Duration::from_millis(10)) {
            // println!("running: {}", ffi::get_id(task.clone()));
            ffi::run_task(task);
            counter = 0;
        }
        map_interface.pin_mut().drawFrame();

        frame.finish().unwrap();
        counter += 1;
        if counter > 50 {
            break;
        }
    }

    let frame = display.draw();
    map_interface.pin_mut().drawFrame();
    map_interface.pin_mut().drawFrame();

    println!("finishing frame");

    frame.finish().unwrap();

    let image: glium::texture::RawImage2d<'_, u8> = display.read_front_buffer().unwrap();

    let image =
        image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
    let image = image::DynamicImage::ImageRgba8(image).flipv();

    image.save("glium-example-screenshot.png").unwrap();
}

use autocxx::prelude::*;
use cxx::{
    private::{SharedPtrTarget, UniquePtrTarget},
    SharedPtr,
};
include_cpp! {
    #include "CoordinateSystemIdentifiers.h"
    #include "graphics/Renderer.h"
    #include "map/MapScene.h"
    #include "MapsCoreSharedModule.h"
    #include "MapInterface.h"
    #include "MapConfig.h"
    #include "SchedulerInterface.h"
    #include "MapCoordinateSystem.h"
    #include "RectCoord.h"
    #include "Coord.h"
    #include "Color.h"
    #include "TaskInterface.h"
    #include "TextureHolderInterface.h"
    #include "TextureLoaderResult.h"
    #include "DataLoaderResult.h"
    #include "make_shared.h"
    #include "CoordinateSystemFactory.h"
    #include "LoaderInterfaceImpl.h"
    #include "LayerInterface.h"
    #include "Tiled2dMapLayerConfig.h"
    #include "Tiled2dMapRasterLayerInterface.h"
    #include "Tiled2dMapRasterLayerInterfaceBuilder.h"
    #include "Tiled2dMapZoomInfo.h"
    #include "MapCamera2dInterface.h"
    #include "Tiled2dMapZoomLevelInfo.h"
    #include "MapCallbackInterface.h"
    #include "PolygonLayerInterface.h"
    #include "PolygonInfo.h"
    #include "PolygonCoord.h"
    #include "Vec2I.h"
    #include "SchedulerInterfaceStaticWrapper.h"
    #include "MapReadyCallbackInterface.h"
    #include "LayerReadyState.h"

    #include "Tiled2dMapLayerConfigWrapper.h"

    safety!(unsafe_ffi)
    generate!("PolygonCoordBuilder")
    generate!("LayerReadyState")
    generate!("PolygonCoord")
    generate!("MapCallbackInterface")
    generate!("PolygonLayerInterface")
    generate!("CoordinateSystemIdentifiers")
    generate!("MapsCoreSharedModule")
    generate!("Renderer")
    generate!("MapInterface")
    generate!("MapCamera2dInterface")
    generate!("MapConfig")
    generate!("TaskInterface")
    generate!("MapCoordinateSystem")
    generate!("CoordinateSystemFactory")
    generate!("RectCoord")
    generate!("SchedulerInterface")
    generate!("Tiled2dMapZoomInfo")
    generate!("Tiled2dMapZoomLevelInfo")
    generate!("LoaderInterfaceImpl")
    generate!("DataLoaderResult")
    generate!("TextureLoaderResult")
    generate!("TextureHolderInterface")
    generate!("Tiled2dMapLayerConfig")
    generate!("Tiled2dMapRasterLayerInterface")
    generate!("Tiled2dMapRasterLayerInterfaceBuilder")
    generate!("Coord")
    generate!("Color")
    generate!("Vec2I")
    generate!("LayerInterface")
    generate!("transform_unique")
    generate!("transform_texture_holder_interface")
    generate!("make_loader_result")
    generate!("down_cast_to_layer_interface")
    generate!("make_vec_zoom_level_info")
    generate!("add_zoom_level_info")
    generate!("run_task")
    generate!("get_id")
    generate!("is_graphics")
    generate!("PolygonInfo")
    generate!("to_map_callback_interface_shared_pointer")
    generate!("make_polygon_coord")
    generate!("transform_ready_state")
    generate!("MapScene")
    // subclass!("SchedulerInterfaceStaticWrapper", SchedulerInterfaceImpl)
    subclass!("LoaderInterfaceImpl", LoaderInterfaceImplRs)
    subclass!("TextureHolderInterface", TextureHolderInterfaceImpl)
    // subclass!("Tiled2dMapLayerConfigWrapper", Tiled2dMapLayerConfigWrapperImpl)
    subclass!("MapCallbackInterface", MapCallbackInterfaceImpl)

    subclass!("MapReadyCallbackInterface", MapReadyCallbackInterfaceImpl)
    generate!("Tiled2dMapLayerConfigWrapper")
    generate!("SchedulerInterfaceStaticWrapper")

}

use autocxx_macro::{extern_rust_function, subclass};

#[subclass(superclass("MapReadyCallbackInterface"))]
#[derive(Default)]
pub struct MapReadyCallbackInterfaceImpl {
    sender: Option<Sender<ffi::LayerReadyState>>,
}

impl ffi::MapReadyCallbackInterface_methods for MapReadyCallbackInterfaceImpl {
    fn stateDidUpdate(&mut self, state: ffi::LayerReadyState) {
        match state {
            ffi::LayerReadyState::READY => println!("READY"),
            ffi::LayerReadyState::NOT_READY => {}
            ffi::LayerReadyState::ERROR => println!("ERROR"),
            ffi::LayerReadyState::TIMEOUT_ERROR => println!("TIMEOUT_ERROR"),
        }
        if let Some(sender) = self.sender.as_ref() {
            let _ = sender.send(state);
        }
    }
}

#[subclass(superclass("MapCallbackInterface"))]
#[derive(Default)]
pub struct MapCallbackInterfaceImpl {
    sender: Option<Sender<()>>,
}

impl ffi::MapCallbackInterface_methods for MapCallbackInterfaceImpl {
    fn invalidate(&mut self) {
        if let Some(sender) = self.sender.as_ref() {
            sender.send(());
        }
    }
}

pub fn new_task_interface() -> Box<SchedulerInterfaceRust> {
    Box::new(SchedulerInterfaceRust {})
}

// #[subclass(superclass("Tiled2dMapLayerConfigWrapper"))]
#[derive(Default)]
pub struct Tiled2dMapLayerConfigWrapperImpl;

impl Tiled2dMapLayerConfigWrapperImpl {
    fn getCoordinateSystemIdentifier(&self) -> cxx::UniquePtr<cxx::CxxString> {
        ffi::CoordinateSystemIdentifiers::EPSG3857()
    }

    fn getTileUrl(&self, x: i32, y: i32, t: i32, zoom: i32) -> cxx::UniquePtr<cxx::CxxString> {
        // println!("getTIle url");
        let the_url = format!("https://a.tile.openstreetmap.org/{zoom}/{x}/{y}.png");
        // println!("{the_url}");
        ffi::make_string(&the_url)
    }

    fn getZoomLevelInfos(&self) -> cxx::UniquePtr<cxx::CxxVector<ffi::Tiled2dMapZoomLevelInfo>> {
        let mut zoom_infos = ffi::make_vec_zoom_level_info();
        let epsg3857Bounds = ffi::RectCoord::new(
            ffi::Coord::new(
                ffi::CoordinateSystemIdentifiers::EPSG3857(),
                -20037508.34,
                20037508.34,
                0.0,
            )
            .within_unique_ptr(),
            ffi::Coord::new(
                ffi::CoordinateSystemIdentifiers::EPSG3857(),
                20037508.34,
                -20037508.34,
                0.0,
            )
            .within_unique_ptr(),
        )
        .within_unique_ptr();
        ffi::add_zoom_level_info(
            zoom_infos.pin_mut(),
            ffi::Tiled2dMapZoomLevelInfo::new(
                559082264.029,
                40075016.0,
                1,
                1,
                1,
                0,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        ffi::add_zoom_level_info(
            zoom_infos.pin_mut(),
            ffi::Tiled2dMapZoomLevelInfo::new(
                279541132.015,
                20037508.0,
                2,
                2,
                1,
                1,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        ffi::add_zoom_level_info(
            zoom_infos.pin_mut(),
            ffi::Tiled2dMapZoomLevelInfo::new(
                139770566.007,
                10018754.0,
                4,
                4,
                1,
                2,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        ffi::add_zoom_level_info(
            zoom_infos.pin_mut(),
            ffi::Tiled2dMapZoomLevelInfo::new(
                69885283.0036,
                5009377.1,
                8,
                8,
                1,
                3,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        ffi::add_zoom_level_info(
            zoom_infos.pin_mut(),
            ffi::Tiled2dMapZoomLevelInfo::new(
                34942641.5018,
                2504688.5,
                16,
                16,
                1,
                4,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        ffi::add_zoom_level_info(
            zoom_infos.pin_mut(),
            ffi::Tiled2dMapZoomLevelInfo::new(
                17471320.7509,
                1252344.3,
                32,
                32,
                1,
                5,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        ffi::add_zoom_level_info(
            zoom_infos.pin_mut(),
            ffi::Tiled2dMapZoomLevelInfo::new(
                4367830.18773,
                313086.1,
                128,
                128,
                1,
                7,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        zoom_infos
    }

    fn getZoomInfo(&self) -> cxx::UniquePtr<ffi::Tiled2dMapZoomInfo> {
        ffi::Tiled2dMapZoomInfo::new(1.0, 0, true, false, true, true).within_unique_ptr()
    }

    fn getLayerName(&self) -> cxx::UniquePtr<cxx::CxxString> {
        ffi::make_string("fancy_pancy")
    }

    // fn getVectorSettingsWrapped(&self) -> cxx::UniquePtr<ffi::Tiled2dMapVectorSettings> {
    //     cxx::UniquePtr::null()
    // }
}

#[subclass(superclass("TextureHolderInterface"))]
#[derive(Default)]
pub struct TextureHolderInterfaceImpl {
    image_width: usize,
    image_height: usize,
    image_data: Vec<u8>,
    texture_data: Vec<u8>,
    usage_counter: usize,
    id: u32,
    attached: bool,
    texture: Option<glium::texture::SrgbTexture2d>,
    display: Option<glium::Display>,
}

impl ffi::TextureHolderInterface_methods for TextureHolderInterfaceImpl {
    fn getImageWidth(&mut self) -> i32 {
        self.image_width as i32
    }

    fn getImageHeight(&mut self) -> i32 {
        self.image_height as i32
    }

    fn getTextureWidth(&mut self) -> i32 {
        self.image_width as i32
    }

    fn getTextureHeight(&mut self) -> i32 {
        self.image_height as i32
    }

    fn attachToGraphics(&mut self) -> i32 {
        if !self.attached {
            if self.image_data.is_empty() {
                println!("WAAAAAH NO IMAGE DATA");

                return 0;
            }

            unsafe {
                let internal_format = gl::RGBA;
                unsafe {
                    gl::GenTextures(1, &mut self.id);
                    gl::BindTexture(gl::TEXTURE_2D, self.id);

                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        gl::LINEAR_MIPMAP_LINEAR as i32,
                    );
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        internal_format as i32,
                        self.image_width as i32,
                        self.image_height as i32,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        self.texture_data.as_ptr() as *const _,
                    );
                    gl::GenerateMipmap(gl::TEXTURE_2D);
                }
            };
            // let image = glium::texture::RawImage2d::from_raw_rgba(
            //     self.texture_data.clone(),
            //     (self.image_width as u32, self.image_height as u32),
            // );
            // let Some(display) = self.display.as_ref() else {
            //     return 0;
            // };
            // let t = glium::texture::SrgbTexture2d::new(display, image).unwrap();
            // let uniforms = uniform! {
            //     sampler: t.sampled().wrap_function(SamplerWrapFunction::BorderClamp)
            // };

            // use glium::GlObject;
            // self.id = t.get_id();
            // self.texture = Some(t);

            self.attached = true;
        }
        self.usage_counter += 1;
        self.id as i32
    }

    fn clearFromGraphics(&mut self) {
        // println!("");
        // println!("Clear texture");
        if self.usage_counter == 0 {
            self.attached = false;
            // unsafe { gl::DeleteTextures(1, &mut self.id) };
            // println!("Last Usage cleaning up");
        } else {
            self.usage_counter -= 1;
            // println!("Reducing usage count");
        }
    }
}

#[subclass(superclass("LoaderInterfaceImpl"))]
#[derive(Default)]
pub struct LoaderInterfaceImplRs {
    display: Option<glium::Display>,
}

impl LoaderInterfaceImpl_methods for LoaderInterfaceImplRs {
    fn loadTextureWrapper(
        &self,
        url: &cxx::CxxString,
        etag: cxx::UniquePtr<cxx::CxxString>,
    ) -> cxx::UniquePtr<ffi::TextureLoaderResult> {
        // println!("In load texture interface");
        let Ok(data) = ureq::get(url.to_str().unwrap()).call() else {
            let load_result = TextureHolderInterfaceImpl::default_cpp_owned();
            let tex_holder_iface =
            TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);
            let tex_holder_iface = ffi::transform_texture_holder_interface(tex_holder_iface);
            return ffi::make_loader_result(tex_holder_iface, ffi::LoaderStatus::ERROR_OTHER);
        };
        let mut databytes = vec![];
        let Ok(_)=  data.into_reader().read_to_end(&mut databytes) else {
            let load_result = TextureHolderInterfaceImpl::default_cpp_owned();
            let tex_holder_iface =
            TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);
            let tex_holder_iface = ffi::transform_texture_holder_interface(tex_holder_iface);
            return ffi::make_loader_result(tex_holder_iface, ffi::LoaderStatus::ERROR_OTHER);
        };
        let Ok(image) = image::load_from_memory(&databytes) else {
            let load_result = TextureHolderInterfaceImpl::default_cpp_owned();
            let tex_holder_iface =
            TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);
            let tex_holder_iface = ffi::transform_texture_holder_interface(tex_holder_iface);
            return ffi::make_loader_result(tex_holder_iface, ffi::LoaderStatus::ERROR_OTHER);
        };
        let image_dimensions = image.dimensions();
        let img_buffer = image.to_rgba8();
        let mut interface = TextureHolderInterfaceImpl {
            image_width: image_dimensions.0 as usize,
            image_height: image_dimensions.1 as usize,
            image_data: databytes,
            texture_data: img_buffer.to_vec(),
            display: Some(self.display.as_ref().unwrap().clone()),

            ..Default::default()
        };
        let mut load_result = TextureHolderInterfaceImpl::new_cpp_owned(interface);
        let tex_holder_iface =
            TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);

        let tex_holder_iface = ffi::transform_texture_holder_interface(tex_holder_iface);
        ffi::make_loader_result(tex_holder_iface, ffi::LoaderStatus::OK)
    }

    fn loadDataWrapper(
        &self,
        url: &cxx::CxxString,
        etag: cxx::UniquePtr<cxx::CxxString>,
    ) -> cxx::UniquePtr<ffi::DataLoaderResult> {
        todo!()
    }
}

unsafe impl Send for ffi::TaskInterface {}
unsafe impl Sync for ffi::TaskInterface {}

mod SchedulerInterfaceImplPool {
    lazy_static::lazy_static! {
       pub static ref STATIC_RUNTIME_POOL : std::sync::Mutex< (Option<std::sync::mpsc::Sender<cxx::SharedPtr<super::ffi::TaskInterface>>>, tokio::runtime::Runtime)> = {
            std::sync::Mutex::new((None, tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .max_blocking_threads(512)
            .thread_keep_alive(std::time::Duration::from_secs(30))
            .build()
            .unwrap()))
        };
    }
}
// #[autocxx::extern_rust::extern_rust_type]
pub struct SchedulerInterfaceRust {}

// #[autocxx::extern_rust::extern_rust_function]

#[cxx::bridge]
mod custom {
    extern "Rust" {
        type SchedulerInterfaceRust;
        fn addTaskRust(&self, task: SharedPtr<TaskInterface>);
        fn removeTaskRust(&self, id: String);
        fn clearRust(&self);
        fn resumeRust(&self);
        fn pauseRust(&self);
        fn new_task_interface() -> Box<SchedulerInterfaceRust>;
    }
    extern "C++" {
        include!("TaskInterface.h");
        type TaskInterface = super::ffi::TaskInterface;
    }
}

pub fn new_layer_config_inner_wrapper() -> Box<Tiled2dMapLayerConfigWrapperImpl> {
    Box::new(Tiled2dMapLayerConfigWrapperImpl)
}
use autocxx::cxx;

#[cxx::bridge]
mod Tiled2dMapLayerConfigWrapperImplMod {
    extern "Rust" {
        type Tiled2dMapLayerConfigWrapperImpl;
        fn getCoordinateSystemIdentifier(&self) -> UniquePtr<CxxString>;
        fn getTileUrl(&self, x: i32, y: i32, t: i32, zoom: i32) -> UniquePtr<CxxString>;
        fn getZoomLevelInfos(&self) -> UniquePtr<CxxVector<Tiled2dMapZoomLevelInfo>>;
        fn getZoomInfo(&self) -> UniquePtr<Tiled2dMapZoomInfo>;
        fn getLayerName(&self) -> UniquePtr<CxxString>;
        fn new_layer_config_inner_wrapper() -> Box<Tiled2dMapLayerConfigWrapperImpl>;
    }
    extern "C++" {
        include!("Tiled2dMapLayerConfigWrapper.h");
        include!("Tiled2dMapZoomLevelInfo.h");
        include!("Tiled2dMapZoomInfo.h");
        type Tiled2dMapZoomLevelInfo = super::ffi::Tiled2dMapZoomLevelInfo;
        type Tiled2dMapZoomInfo = super::ffi::Tiled2dMapZoomInfo;
    }
}

use ffi::TaskInterface;
// #[autocxx::extern_rust::extern_rust_function]
impl SchedulerInterfaceRust {
    // #[autocxx::extern_rust::extern_rust_function]
    fn addTaskRust(&self, task: SharedPtr<TaskInterface>) {
        let t = task.clone();
        if !ffi::is_graphics(t.clone()) {
            let spawner = SchedulerInterfaceImplPool::STATIC_RUNTIME_POOL
                .lock()
                .unwrap();
            spawner.1.spawn_blocking(move || {
                // println!("running: {}", ffi::get_id(t.clone()));
                ffi::run_task(t.clone());
                // println!("finished: {}", ffi::get_id(t.clone()));
            });
        } else if let Ok(sender) = SchedulerInterfaceImplPool::STATIC_RUNTIME_POOL.lock() {
            if let Some(sender) = sender.0.as_ref() {
                let _ = sender.send(t);
            }
        }
    }
    // #[autocxx::extern_rust::extern_rust_function]
    fn removeTaskRust(&self, id: String) {
        println!("removeTask")
    }
    // #[autocxx::extern_rust::extern_rust_function]
    fn clearRust(&self) {
        println!("clear")
    }
    // #[autocxx::extern_rust::extern_rust_function]
    fn pauseRust(&self) {
        println!("pause")
    }
    // #[autocxx::extern_rust::extern_rust_function]
    fn resumeRust(&self) {
        println!("resume")
    }
}
