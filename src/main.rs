use autocxx::{cxx::SharedPtr, subclass::prelude::*};

use glutin::{
    dpi::{PhysicalSize, Size},
    ContextBuilder
};

use openmobilemaps_bindings::bindings::impls::{
    LoaderInterfaceImplRs, MapCallbackInterfaceImpl, MapReadyCallbackInterfaceImpl,
};

use std::{
    default::Default,
    pin::Pin,
    time::Duration,
};

use openmobilemaps_sys::openmobilemaps_bindings::*;

fn main() {
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
    let _ = frame.finish();


    let mut arrays = 0;
    unsafe { gl::GenVertexArrays(10, &mut arrays) };
    unsafe { gl::BindVertexArray(arrays) };

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let coordsystem = CoordinateSystemFactory::getEpsg3857System();
    let map_config = MapConfig::new(coordsystem.within_unique_ptr()).within_unique_ptr();

    let (tx, rx) = std::sync::mpsc::channel();
    SchedulerInterfaceImplPool::STATIC_RUNTIME_POOL
        .lock()
        .unwrap()
        .0 = Some(tx);

    let scheduler = SchedulerInterfaceStaticWrapper::new().within_unique_ptr();
    let scheduler = transform_unique(scheduler);
    // println!("{}", shared_ptr.is_null());
    let map_interface: SharedPtr<MapInterface> =
        MapInterface::createWithOpenGl(&map_config, &scheduler, 1.0);
    let (invalidate_sender, invalidate_receiver) = std::sync::mpsc::channel();

    let mut callbacks = MapCallbackInterfaceImpl::default();
    callbacks.sender = Some(invalidate_sender);
    let callbacks = MapCallbackInterfaceImpl::new_cpp_owned(callbacks);
    let callbackInterface = MapCallbackInterfaceImpl::as_MapCallbackInterface_unique_ptr(callbacks);

    pin_mut!(map_interface)
        .setCallbackHandler(&to_map_callback_interface_shared_pointer(callbackInterface));
    let (ready_state_sender, ready_state_receiver) = std::sync::mpsc::channel();
    let mut ready_state = MapReadyCallbackInterfaceImpl::default();
    ready_state.sender = Some(ready_state_sender);

    let ready_state = MapReadyCallbackInterfaceImpl::new_cpp_owned(ready_state);
    let ready_state_interface =
        MapReadyCallbackInterfaceImpl::as_MapReadyCallbackInterface_unique_ptr(ready_state);

    let mut loader = LoaderInterfaceImplRs::default();
    loader.display = Some(display.clone());
    let loader = LoaderInterfaceImplRs::new_cpp_owned(loader);

    let loader = LoaderInterfaceImplRs::as_LoaderInterfaceImpl_unique_ptr(loader);

    println!("start downcast");
    let loader = LoaderInterfaceImpl::toShared(loader);

    let loader = LoaderInterfaceImpl::asLoaderInterface(loader);
    println!("downcast succeeded");

    println!("LoaderInterface: {}", loader.is_null());

    let mut builder = Tiled2dMapRasterLayerInterfaceBuilder::builder().within_unique_ptr();
    println!("add loader");
    builder.pin_mut().addLoader(loader);

    println!("added loader");
    let config_wrapper = Tiled2dMapLayerConfigWrapper::new().within_unique_ptr();
    // builder.pin_mut().addConfig();

    let config = Tiled2dMapLayerConfigWrapper::asTiled2dMapLayerConfig(config_wrapper);

    builder.pin_mut().setConfig(config);
    let mut tiled = builder.pin_mut().build();

    let polygon_layer = PolygonLayerInterface::create();
    let mut polygon_coord_builder = PolygonCoordBuilder::new().within_unique_ptr();
    polygon_coord_builder.pin_mut().addCoord(
        &Coord::new(
            CoordinateSystemIdentifiers::EPSG2056(),
            2684200.0,
            1244833.3,
            0.0,
        )
        .within_unique_ptr(),
    );
    polygon_coord_builder.pin_mut().addCoord(
        &Coord::new(
            CoordinateSystemIdentifiers::EPSG2056(),
            2684200.0,
            1345833.3,
            0.0,
        )
        .within_unique_ptr(),
    );
    polygon_coord_builder.pin_mut().addCoord(
        &Coord::new(
            CoordinateSystemIdentifiers::EPSG2056(),
            2785200.0,
            1345833.3,
            0.0,
        )
        .within_unique_ptr(),
    );
    polygon_coord_builder.pin_mut().addCoord(
        &Coord::new(
            CoordinateSystemIdentifiers::EPSG2056(),
            2785200.0,
            1244833.3,
            0.0,
        )
        .within_unique_ptr(),
    );

    let polygon_coord = polygon_coord_builder.pin_mut().build();
    let polygon_info = PolygonInfo::new(
        "test",
        polygon_coord,
        Color::new(1.0, 0.0, 0.0, 1.0).within_unique_ptr(),
        Color::new(1.0, 0.4, 0.4, 1.0).within_unique_ptr(),
    )
    .within_unique_ptr();
    let p = unsafe { cxx_shared_cast(polygon_layer.as_ref().unwrap()) };
    p.add(&polygon_info);

    let mut polygon_coord_builder = PolygonCoordBuilder::new().within_unique_ptr();
    polygon_coord_builder.pin_mut().addCoord(
        &Coord::new(
            CoordinateSystemIdentifiers::EPSG2056(),
            2694200.0,
            1254833.3,
            0.0,
        )
        .within_unique_ptr(),
    );
    polygon_coord_builder.pin_mut().addCoord(
        &Coord::new(
            CoordinateSystemIdentifiers::EPSG2056(),
            2694200.0,
            1355833.3,
            0.0,
        )
        .within_unique_ptr(),
    );
    polygon_coord_builder.pin_mut().addCoord(
        &Coord::new(
            CoordinateSystemIdentifiers::EPSG2056(),
            2795200.0,
            1355833.3,
            0.0,
        )
        .within_unique_ptr(),
    );
    polygon_coord_builder.pin_mut().addCoord(
        &Coord::new(
            CoordinateSystemIdentifiers::EPSG2056(),
            2795200.0,
            1254833.3,
            0.0,
        )
        .within_unique_ptr(),
    );

    let polygon_coord = polygon_coord_builder.pin_mut().build();
    let polygon_info = PolygonInfo::new(
        "test",
        polygon_coord,
        Color::new(0.0, 1.0, 0.0, 1.0).within_unique_ptr(),
        Color::new(1.0, 0.4, 0.4, 1.0).within_unique_ptr(),
    )
    .within_unique_ptr();
    let p = unsafe { cxx_shared_cast(polygon_layer.as_ref().unwrap()) };
    p.add(&polygon_info);

    let p = unsafe { cxx_shared_cast(polygon_layer.as_ref().unwrap()) };
    let the_layer = p.asLayerInterface();
    pin_mut!(map_interface).addLayer(&the_layer);

    pin_mut!(map_interface).setViewportSize(&Vec2I::new(1200, 900).within_unique_ptr());

    println!("before cast as layerInterface");
    let blub = down_cast_to_layer_interface(tiled);
    println!("after cast as layerInterface");

    pin_mut!(map_interface).addLayer(&blub);
    // let loader_list = get_loader_list(tiled);

    println!("layer added");

    let red_color = Color::new(1.0, 1.0, 1.0, 0.5).within_unique_ptr();
    pin_mut!(map_interface).setBackgroundColor(&red_color);
    let camera = pin_mut!(map_interface).getCamera();

    let c: Pin<&mut MapCamera2dInterface> = unsafe { cxx_const_cast(camera.as_ref().unwrap()) };

    c.setMaxZoom(0.0);
    let c: Pin<&mut MapCamera2dInterface> = unsafe { cxx_const_cast(camera.as_ref().unwrap()) };
    c.setMinZoom(f64::MAX);

    let centerCoord = Coord::new(
        CoordinateSystemIdentifiers::EPSG4326(),
        8.543912536386152,
        47.37623511643675,
        0.0,
    )
    .within_unique_ptr();

    let c: Pin<&mut MapCamera2dInterface> = unsafe { cxx_const_cast(camera.as_ref().unwrap()) };

    c.moveToCenterPositionZoom(&centerCoord, 50000.0, false);

    pin_mut!(map_interface).resume();
    pin_mut!(map_interface).drawFrame();

    let mut counter = 0;
    loop {
        let frame = display.draw();
        if let Ok(task) = rx.recv_timeout(Duration::from_millis(10)) {
            // println!("running: {}", get_id(task.clone()));
            run_task(task);
            counter = 0;
        }
        if let Ok(_) = invalidate_receiver.recv_timeout(Duration::from_millis(1)) {
             pin_mut!(map_interface).invalidate();
        }
        pin_mut!(map_interface).drawFrame();

        frame.finish().unwrap();
        counter += 1;
        if counter > 50 {
            break;
        }
    }

    let frame = display.draw();
    pin_mut!(map_interface).drawFrame();
    pin_mut!(map_interface).drawFrame();

    println!("finishing frame");

    frame.finish().unwrap();

    let image: glium::texture::RawImage2d<'_, u8> = display.read_front_buffer().unwrap();

    let image =
        image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
    let image = image::DynamicImage::ImageRgba8(image).flipv();

    image.save("glium-example-screenshot.png").unwrap();
}
