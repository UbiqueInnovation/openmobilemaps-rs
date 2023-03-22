use euclid::Size2D;
use geo_types::LineString;
use image::{GenericImageView, ImageFormat, Rgba, RgbaImage};
use imageproc::drawing::{
    draw_filled_circle_mut, draw_filled_rect_mut, draw_polygon_mut, draw_text_mut, text_size,
};
use resvg::{tiny_skia, usvg};
use rusttype::{Font, Scale};
use serde::Deserialize;
use surfman::{
    Connection, ContextAttributeFlags, ContextAttributes, GLVersion, SurfaceAccess, SurfaceType,
};

use std::{
    default::Default,
    io::Cursor,
    time::{Duration, Instant},
};

use openmobilemaps_sys::openmobilemaps_bindings::{
    autocxx::subclass::CppSubclass,
    bindings::{
        external_types::{LoaderInterfaceWrapperImpl, Tiled2dMapLayerConfigWrapperImpl},
        impls::{
            IconInfoInterfaceImpl, MapCallbackInterfaceImpl, MapReadyCallbackInterfaceImpl,
            MAP_READY_CALLBACK,
        },
    },
    cxx::{CxxVector, SharedPtr},
    *,
};
#[macro_export]
macro_rules! html_hex {
    ($html_hex:expr) => {
        'l: {
            let Some(inner) = $html_hex.strip_prefix("#") else {
                                                                             break 'l [0,0,0,1];
                                                                        };
            let bs = if inner.len() == 3 {
                inner
                    .as_bytes()
                    .iter()
                    .map(|s| {
                        u8::from_str_radix(std::str::from_utf8(&[*s, *s]).unwrap(), 16).unwrap()
                    })
                    .collect::<Vec<_>>()
            } else {
                inner
                    .as_bytes()
                    .chunks_exact(2)
                    .map(|s| u8::from_str_radix(std::str::from_utf8(s).unwrap(), 16).unwrap())
                    .collect::<Vec<_>>()
            };
            if bs.len() == 3 {
                [bs[0], bs[1], bs[2], 255]
            } else {
                [bs[0], bs[1], bs[2], bs[3]]
            }
        }
    };
}

#[derive(Deserialize)]
pub struct MeetweenConnections {
    pub stations: Vec<Station>,
    pub workspaces: Vec<Workspace>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    connections: Vec<ViadiConnection>,
    pub meeting_point: Place,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViadiConnection {
    starting_point: String,
    connection: Option<InnerConnection>,
}
#[derive(Deserialize)]
pub struct InnerConnection {
    sections: Vec<Section>,
}
#[derive(Deserialize)]
pub struct Section {
    route: Option<Route>,
    polyline: Option<String>,
}
#[derive(Deserialize)]
pub struct Route {
    icon: u8,
}
#[derive(Deserialize)]
pub struct Place {
    pub name: String,
    coordinate: Coordinate,
}
#[derive(Deserialize)]
pub struct Coordinate {
    lat: f64,
    lon: f64,
}
#[derive(Deserialize)]
pub struct Workspace {}

pub fn draw_map(station: &Station) -> Vec<u8> {
    let colors_array: [[[u8; 4]; 2]; 6] = [
        [html_hex!("#FF5157"), html_hex!("#FFF")],
        [html_hex!("#6474A6"), html_hex!("#FFF")],
        [html_hex!("#3FC67C"), html_hex!("#FFF")],
        [html_hex!("#232443"), html_hex!("#FFF")],
        [html_hex!("#F4C10C"), html_hex!("#000")],
        [html_hex!("#AF5F01"), html_hex!("#FFF")],
    ];

    let start = Instant::now();
    println!("Setup opengl");
    let (display, mut context) = setup_opengl();

    println!("Create map");
    let (rx, map_interface, invalidate_receiver, ready_state_interface, ready_state_receiver) =
        setup_map();
    println!("Add raster layer");
    let (_loader_interface_ptr, raster_layer) = create_raster_layer();

    let line_layer = LineLayerInterface::create();
    let icon_layer = IconLayerInterface::create();
    let mut all_points = vec![(
        station.meeting_point.coordinate.lon,
        station.meeting_point.coordinate.lat,
    )];

    for (i, viadi_connection) in station.connections.iter().enumerate() {
        let color = colors_array[i % colors_array.len()][0];
        let font_color = colors_array[i % colors_array.len()][1];
        let connection_color = Color::new(
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
            color[3] as f32 / 255.0,
        )
        .within_box();
        let Some(poly_line) = viadi_connection.connection.as_ref() else {
            continue;
        };
        let poly_line = poly_line
            .sections
            .iter()
            .filter_map(|s| s.polyline.as_ref())
            .filter_map(|section| polyline::decode_polyline(&section, 5).ok())
            .collect::<Vec<_>>();
        let mut color_outer = color;
        color_outer[3] = 100;
        let (points, connection_interface) = new_poly_line(&poly_line, &connection_color);
        pin_mut!(line_layer).add(&connection_interface);
        all_points.extend(&points);

        let mut the_icon = IconInfoInterfaceImpl::default();
        the_icon.texture_data =
            get_start_point(&(i + 1).to_string(), font_color, color_outer, color, 80, 80);
        the_icon.image_width = 80;
        the_icon.image_height = 80;
        the_icon.anchor = (0.5, 0.5);
        the_icon.coordinate = (
            CoordinateSystemIdentifiers::EPSG4326()
                .to_string_lossy()
                .as_ref()
                .to_string(),
            points[0].0,
            points[0].1,
        );
        let the_icon = IconInfoInterfaceImpl::new_cpp_owned(the_icon);
        let the_icon = IconInfoInterfaceImpl::as_IconInfoInterface_unique_ptr(the_icon);
        let the_icon = transform_icon_info_interface(the_icon);
        pin_mut!(icon_layer).add(&the_icon);
    }
    let destination = (
        station.meeting_point.coordinate.lon,
        station.meeting_point.coordinate.lat,
        station.meeting_point.name.clone(),
    );
    let mut the_icon = IconInfoInterfaceImpl::default();
    let (icon_width, icon_height, texture_data) = get_destination_box(
        &destination.2,
        station
            .connections
            .last()
            .and_then(|a| a.connection.as_ref())
            .and_then(|a| a.sections.last())
            .and_then(|a| a.route.as_ref())
            .map(|a| a.icon)
            .unwrap_or(1),
    );
    the_icon.texture_data = texture_data;
    the_icon.image_width = icon_width as usize;
    the_icon.image_height = icon_height as usize;
    the_icon.anchor = (0.5, 1.0);
    the_icon.coordinate = (
        CoordinateSystemIdentifiers::EPSG4326()
            .to_string_lossy()
            .as_ref()
            .to_string(),
        destination.0,
        destination.1,
    );
    let the_icon = IconInfoInterfaceImpl::new_cpp_owned(the_icon);
    let the_icon = IconInfoInterfaceImpl::as_IconInfoInterface_unique_ptr(the_icon);
    let the_icon = transform_icon_info_interface(the_icon);
    pin_mut!(icon_layer).add(&the_icon);

    pin_mut!(map_interface).addLayer(&raster_layer);

    let line_layer = pin_mut!(line_layer).asLayerInterface();
    pin_mut!(map_interface).addLayer(&line_layer);

    let icon_layer = pin_mut!(icon_layer).asLayerInterface();
    pin_mut!(map_interface).addLayer(&icon_layer);

    println!("Setup camera");
    let camera = pin_mut!(map_interface).getCamera();
    pin_mut!(camera).setMaxZoom(0.0);
    pin_mut!(camera).setMinZoom(f64::MAX);

    let center_coord = Coord::new(
        CoordinateSystemIdentifiers::EPSG4326(),
        destination.0,
        destination.1,
        0.0,
    )
    .within_unique_ptr();

    pin_mut!(map_interface).resume();
    pin_mut!(camera).moveToCenterPosition(&center_coord, false);
    pin_mut!(camera).setPaddingBottom(178.0);
    pin_mut!(camera).setPaddingLeft(100.0);
    pin_mut!(camera).setPaddingRight(100.0);
    pin_mut!(camera).setPaddingTop(100.0);

    pin_mut!(map_interface).setBackgroundColor(&Color::new(0.0, 0.0, 0.0, 1.0).within_unique_ptr());

    let b = BoundingBox::new1(&CoordinateSystemIdentifiers::EPSG4326()).within_unique_ptr();
    for (x, y) in all_points {
        pin_mut!(b).addPoint(x, y, 0.0);
    }

    let ready_state_interface = transform_ready_state(ready_state_interface);

    let map_interface2 = map_interface.clone();
    let bounds = pin_mut!(b).asRectCoord().within_unique_ptr();
    std::thread::spawn(move || {
        let map_interface = map_interface2;
        pin_mut!(map_interface).drawReadyFrame(&bounds, 10.0, &ready_state_interface);
    });
    //

    println!("Start rendering loop");
    let mut buffer = vec![0u8; 1200 * 630 * 4];
    let mut render_passes = 1;
    display.make_context_current(&context);
    loop {
        if let Ok(task) = rx.recv_timeout(Duration::from_millis(1)) {
            run_task(task);
        }
        if invalidate_receiver
            .recv_timeout(Duration::from_millis(1))
            .is_ok()
        {
            pin_mut!(map_interface).invalidate();
        }

        pin_mut!(map_interface).drawFrame();

        if let Ok(state) = ready_state_receiver.recv_timeout(Duration::from_millis(1)) {
            if state == LayerReadyState::READY {
                unsafe {
                    gl::Finish();
                };

                unsafe {
                    gl::ReadPixels(
                        0,
                        0,
                        1200,
                        630,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        buffer.as_mut_ptr() as _,
                    );
                }
                break;
            }
        }
    }
    println!("finishing frame (after {render_passes} render passes)");
    display.destroy_context(&mut context);

    let image = image::ImageBuffer::from_raw(1200, 630, buffer).unwrap();

    let image = image::DynamicImage::ImageRgba8(image).flipv();
    let mut image = image.resize_exact(1200, 630, image::imageops::FilterType::Lanczos3);

    let bottomstuff = image::open("./bottomstuff.jpeg").unwrap();
    let bottomstuff = bottomstuff.resize_exact(1200, 78, image::imageops::FilterType::Lanczos3);
    let bottomsheet = image.dimensions().1 as i64 - 78;
    image::imageops::replace(&mut image, &bottomstuff, 0, bottomsheet);
    let mut output_buffer = Cursor::new(Vec::with_capacity(1200 * 630));
    image.write_to(&mut output_buffer, ImageFormat::Jpeg);

    let end = Instant::now();
    println!("Took {}ms", (end - start).as_millis());
    output_buffer.into_inner()
}

pub fn setup_opengl() -> (surfman::Device, surfman::Context) {
    let connection = Connection::new().unwrap();
    let adapter = connection.create_adapter().unwrap();
    let mut device = connection.create_device(&adapter).unwrap();
    let context_attributes = ContextAttributes {
        version: GLVersion::new(4, 3),
        flags: ContextAttributeFlags::ALPHA
            | ContextAttributeFlags::STENCIL
            | ContextAttributeFlags::DEPTH,
    };
    let context_descriptor = device
        .create_context_descriptor(&context_attributes)
        .unwrap();
    let mut context = device.create_context(&context_descriptor, None).unwrap();

    let surface = device
        .create_surface(
            &context,
            SurfaceAccess::GPUOnly,
            SurfaceType::Generic {
                size: Size2D::new(1200, 630),
            },
        )
        .unwrap();
    device
        .bind_surface_to_context(&mut context, surface)
        .unwrap();

    device.make_context_current(&context).unwrap();

    gl::load_with(|s| device.get_proc_address(&context, s) as *const std::os::raw::c_void);

    let mut arrays = 0;
    unsafe { gl::GenVertexArrays(1, &mut arrays) };
    unsafe { gl::BindVertexArray(arrays) };

    unsafe {
        gl::Disable(gl::CULL_FACE);
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::BLEND);
        // gl::Enable(gl::MULTISAMPLE);
        let surface_info = device.context_surface_info(&context).unwrap().unwrap();
        gl::BindFramebuffer(gl::FRAMEBUFFER, surface_info.framebuffer_object);
        gl::Viewport(0, 0, 1200, 630);
    }
    // glium::HeadlessRenderer::new(context).unwrap()
    (device, context)
}

pub fn new_poly_line(
    polylines: &[LineString],
    color: &Color,
) -> (Vec<(f64, f64)>, SharedPtr<LineInfoInterface>) {
    let line_layer_info_interface = LineInfoInterfaceWrapperBuilder::new().within_unique_ptr();
    let mut coords = vec![];
    for line in polylines {
        for coord in line.points() {
            pin_mut!(line_layer_info_interface).addCoordinate(
                Coord::new(
                    CoordinateSystemIdentifiers::EPSG4326(),
                    coord.x(),
                    coord.y(),
                    0.0,
                )
                .within_unique_ptr()
                .pin_mut(),
            );
            coords.push((coord.x(), coord.y()));
        }
    }
    let line_style = LineStyle::new(
        ColorStateList::new(color, color).within_box(),
        ColorStateList::new(color, color).within_box(),
        0.8,
        5.0,
        SizeType::SCREEN_PIXEL,
        10.0,
        make_default_dash(),
        LineCapType::ROUND,
    )
    .within_unique_ptr();
    pin_mut!(line_layer_info_interface).setStyle(line_style);
    pin_mut!(line_layer_info_interface).setIdentifier("connection_a");
    println!("Build line_layer");
    (coords, pin_mut!(line_layer_info_interface).build())
}

pub struct ZoomInfo;

impl Tiled2dMapLayerConfigTrait for ZoomInfo {
    fn getCoordinateSystemIdentifier(&self) -> UniquePtr<cxx::CxxString> {
        todo!()
    }

    fn getTileUrl(&self, x: i32, y: i32, t: i32, zoom: i32) -> UniquePtr<cxx::CxxString> {
        // println!("getTIle url");
        let the_url = format!("https://osm-tile-flesk.openmobilemaps.io/{zoom}/{x}/{y}.png");
        // println!("{the_url}");
        make_string(&the_url)
    }

    fn getZoomLevelInfos(&self) -> UniquePtr<CxxVector<Tiled2dMapZoomLevelInfo>> {
        let mut zoom_infos = make_vec_zoom_level_info();
        let epsg3857Bounds = RectCoord::new(
            Coord::new(
                CoordinateSystemIdentifiers::EPSG3857(),
                -20037508.34,
                20037508.34,
                0.0,
            )
            .within_unique_ptr(),
            Coord::new(
                CoordinateSystemIdentifiers::EPSG3857(),
                20037508.34,
                -20037508.34,
                0.0,
            )
            .within_unique_ptr(),
        )
        .within_unique_ptr();
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(559082264.029, 40075016.0, 1, 1, 1, 0, &epsg3857Bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(279541132.015, 20037508.0, 2, 2, 1, 1, &epsg3857Bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(139770566.007, 10018754.0, 4, 4, 1, 2, &epsg3857Bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(69885283.0036, 5009377.1, 8, 8, 1, 3, &epsg3857Bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(34942641.5018, 2504688.5, 16, 16, 1, 4, &epsg3857Bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(17471320.7509, 1252344.3, 32, 32, 1, 5, &epsg3857Bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(4367830.18773, 313086.1, 128, 128, 1, 7, &epsg3857Bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(2183915.09386, 156543.0, 256, 256, 1, 8, &epsg3857Bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(1091957.54693, 78271.5, 512, 512, 1, 9, &epsg3857Bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                545978.773466,
                39135.8,
                1024,
                1024,
                1,
                10,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );

        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                272989.386733,
                19567.9,
                2048,
                2048,
                1,
                11,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );

        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                136494.693366,
                9783.94,
                4096,
                4096,
                1,
                12,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                68247.3466832,
                4891.97,
                8192,
                8192,
                1,
                13,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                34123.6733416,
                2445.98,
                16384,
                16384,
                1,
                14,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                17061.8366708,
                1222.99,
                32768,
                32768,
                1,
                15,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                8530.91833540,
                611.496,
                65536,
                65536,
                1,
                16,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                4265.45916770,
                305.748,
                131072,
                131072,
                1,
                17,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                2132.72958385,
                152.874,
                262144,
                262144,
                1,
                18,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                1066.36479193,
                76.437,
                524288,
                524288,
                1,
                19,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                533.18239597,
                38.2185,
                1048576,
                1048576,
                1,
                20,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        zoom_infos
    }

    fn getZoomInfo(&self) -> UniquePtr<Tiled2dMapZoomInfo> {
        Tiled2dMapZoomInfo::new(1.0, 0, true, false, true, true).within_unique_ptr()
    }

    fn getLayerName(&self) -> UniquePtr<cxx::CxxString> {
        make_string("fancy_pancy")
    }
}

pub fn create_raster_layer() -> (SharedPtr<LoaderInterfaceImpl>, SharedPtr<LayerInterface>) {
    let mut builder = Tiled2dMapRasterLayerInterfaceBuilder::builder().within_unique_ptr();

    let config_wrapper = unsafe {
        let wrapper = Tiled2dMapLayerConfigWrapperImpl(Box::new(ZoomInfo));
        let pointer = Box::into_raw(Box::new(wrapper));
        Tiled2dMapLayerConfigWrapper::new1(pointer as _).within_unique_ptr()
    };
    // builder.pin_mut().addConfig();

    let config = Tiled2dMapLayerConfigWrapper::asTiled2dMapLayerConfig(config_wrapper);

    builder.pin_mut().setConfig(config);

    let loader = LoaderInterfaceWrapperImpl::default();
    let pointer = Box::into_raw(Box::new(loader));
    let loader = unsafe { LoaderInterfaceImpl::new1(pointer as _).within_unique_ptr() };

    let loader = LoaderInterfaceImpl::toShared(loader);

    let loader_shared = LoaderInterfaceImpl::asLoaderInterface(loader.clone());
    builder.pin_mut().addLoader(loader_shared);

    let tiled = builder.pin_mut().build();
    (loader, down_cast_to_layer_interface(tiled))
}

pub fn setup_map() -> (
    std::sync::mpsc::Receiver<SharedPtr<TaskInterface>>,
    SharedPtr<MapInterface>,
    std::sync::mpsc::Receiver<()>,
    UniquePtr<MapReadyCallbackInterface>,
    std::sync::mpsc::Receiver<LayerReadyState>,
) {
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
    let callback_interface =
        MapCallbackInterfaceImpl::as_MapCallbackInterface_unique_ptr(callbacks);

    pin_mut!(map_interface).setCallbackHandler(&to_map_callback_interface_shared_pointer(
        callback_interface,
    ));
    let (ready_state_sender, ready_state_receiver) = std::sync::mpsc::channel();
    let mut guard = MAP_READY_CALLBACK.lock().unwrap();
    *guard = Some(ready_state_sender);
    let mut ready_state = MapReadyCallbackInterfaceImpl::default();
    // ready_state.sender = Some(ready_state_sender);

    let ready_state = MapReadyCallbackInterfaceImpl::new_cpp_owned(ready_state);
    let ready_state_interface =
        MapReadyCallbackInterfaceImpl::as_MapReadyCallbackInterface_unique_ptr(ready_state);

    pin_mut!(map_interface).setViewportSize(&Vec2I::new(1200, 630).within_unique_ptr());
    (
        rx,
        map_interface,
        invalidate_receiver,
        ready_state_interface,
        ready_state_receiver,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_polyline() {
        let poly_line = "eqeaHqiim@vBcJrBwIXkA\\uA^cBJo@Jm@d@qBR{@Hc@Le@n@{BLm@VsAXeBPiAPoANaBFqB@cBEeBMkBK_AMcAQw@o@eCk@qBEO_@oAWeAUcAWyAMs@c@}CMsAImAGmAGcDAuDBcBDaCD}@HyADu@Hu@RcBb@wCj@yCv@gDzCgKlAcEbBsGXkArBeJfAqEpAuFVmAZgB|@{Fb@cC\\_BNo@xAmGdFiTdAiEtAsFhEqQzDmQhByHPu@jAgFvBeJhByHvCcMf@wBhE{QDO|C{Mn@oCr@iDr@eDxAuFPq@n@qBp@qBt@sBx@qB|@oBz@}An@gAZi@`BaCbByB`BaB`B_BzAqA|AeAxAaAfFsDdFiEdMsMnEqGlEqHpFaLlFwLxS{d@``AuuBvH{RpKkYhNi_@lE}JtEiJnG{KrHkLb@m@bDiErA_BnAaBZa@pBiCdByB|@mA~AeCbBoCz@sAhAiBlAqBv@sAhBaDfB_D??xB_ExAkCj@_AbAgBf@aAZi@r@oAp@kAh@cAnByDtA}CvEuLfIuTp@gBtBaGz@aC|@}BhUon@fJoWlAaEd@cB^aBj@yCXqBLoAN_BJ_BJmCBsB?wACyEAc@@{D?sELuDf@_Kr@kKZ_EPwBZwE`GiaAf@sIvAqUvGecAf@qHl@mH~@mFpAoElB_GfAuDd@yBDSn@qDn@yDhF_]r@yF~@eIBS|BmT`@aENcE?_CGiCMyBCYCO]aCMs@WgAQm@[iAUm@a@}@[m@[g@i@{@e@m@q@{@sEyEaC{DmA{Cw@mDYkBEYMuAKkAIaC?mBJiFJ_DHyE?kBGoBSaEOaAWwBK_Ae@yDo@cF}AiMuAiLKw@g@sDOqAS_Ba@iDUmCO_EBeETyDd@iDj@iCRu@b@qA^w@d@_A`AaBV_@^c@hAgAj@c@`Aw@jHoF~BiBtBsBpBgCjBuCrCiGpAwDjAeF`DkR`@gB\\qA^oAv@cCl@uAzA_D~@yA`GiGzR{QX[zByBhAwA`@k@d@o@hBsDl@sAjAgC^_AtAkDjB}D~BaDlBoBdGmFzIkIvBeB`CmArAe@bBi@r@U|Bu@jr@sRjrEalAfoAw\\fuAi\\~IuB`FsAhKmCzHqBxCw@n@Qx@SdB]jAK|@Az@Bz@DpBb@lA^|Ap@`B~@~AnAz@v@v@z@v@~@r@dAp@hAj@fAvA|C`E|JBHTl@j@bB`@bAbAdCr@pAf@v@l@t@l@n@jA|@TNZNXNZLd@Tb@Px@RdAVoB[g@O[C[EZNbAf@dAPn@NlATt@JjEb@??";
        let coords = polyline::decode_polyline(poly_line, 5).unwrap();
        println!("{:?}", coords);
    }
}

pub fn get_start_point(
    number: &str,
    color_text: [u8; 4],
    color_outer: [u8; 4],
    color_inner: [u8; 4],
    output_width: usize,
    output_height: usize,
) -> Vec<u8> {
    let mut image = RgbaImage::new(800, 800);

    draw_filled_circle_mut(&mut image, (400, 400), 400, Rgba(color_outer));
    draw_filled_circle_mut(&mut image, (400, 400), 300, Rgba([255, 255, 255, 255]));
    draw_filled_circle_mut(&mut image, (400, 400), 250, Rgba(color_inner));

    let font = Vec::from(include_bytes!("../AvertaStd-Bold.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let height = 400;
    let scale = Scale {
        x: height as f32,
        y: height as f32,
    };

    let (text_width, text_height) = text_size(scale, &font, number);
    println!("{}/{}", text_width, text_height);
    draw_text_mut(
        &mut image,
        Rgba(color_text),
        390 - text_width / 2,
        390 - text_height / 2,
        scale,
        &font,
        number,
    );
    let image = image::imageops::resize(
        &image,
        output_width as u32,
        output_height as u32,
        image::imageops::FilterType::Lanczos3,
    );
    image.into_vec()
}

fn load_icon(icon: u8) -> Vec<u8> {
    let svg_data = match icon {
        1 => include_str!("../assets/ic_train.svg"),
        2 => include_str!("../assets/ic_bus.svg"),
        3 => include_str!("../assets/ic_schiff.svg"),
        4 => include_str!("../assets/ic_seilbahn.svg"),
        6 => include_str!("../assets/ic_standseilbahn.svg"),
        _ => include_str!("../assets/ic_train.svg"),
    };
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_data, &opt).unwrap();
    let pixmap_size = tree.size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(48, 48).unwrap();
    resvg::render(
        &tree,
        usvg::FitTo::Size(48, 48),
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.data_mut().to_vec()
}

pub fn get_destination_box(destination: &str, icon: u8) -> (i32, i32, Vec<u8>) {
    let font = Vec::from(include_bytes!("../AvertaStd-Bold.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let height = 40;
    let scale = Scale {
        x: height as f32,
        y: height as f32,
    };

    let train_picture = RgbaImage::from_raw(48, 48, load_icon(icon)).unwrap();
    let (text_width, text_height) = text_size(scale, &font, destination);
    let background_color = Rgba([64_u8, 72_u8, 137_u8, 250_u8]);
    let image_width = text_width + 20 + 50;
    let image_height = text_height + 20 + 20;

    let mut image = RgbaImage::new(image_width as u32, image_height as u32);
    let full_rect =
        imageproc::rect::Rect::at(0, 0).of_size(image_width as u32, image_height as u32 - 20);
    draw_filled_rect_mut(&mut image, full_rect, background_color);

    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(0, 0).of_size(20, 20),
        Rgba([0, 0, 0, 0]),
    );
    draw_filled_circle_mut(&mut image, (10, 10), 10, background_color);
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(10, 0).of_size(10, 10),
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(0, 5).of_size(20, 20),
        background_color,
    );

    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 20, 0).of_size(20, 20),
        Rgba([0, 0, 0, 0]),
    );
    draw_filled_circle_mut(&mut image, (image_width - 10, 10), 10, background_color);
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 20, 0).of_size(10, 20),
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 20, 10).of_size(20, 10),
        background_color,
    );

    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(0, image_height - 20 - 20).of_size(20, 20),
        Rgba([0, 0, 0, 0]),
    );
    draw_filled_circle_mut(&mut image, (10, image_height - 20 - 10), 10, background_color);
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(0, image_height - 20 - 20).of_size(20, 10),
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(10, image_height - 20 - 20).of_size(10, 20),
        background_color,
    );

    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 20, image_height - 20 - 20).of_size(20, 20),
        Rgba([0, 0, 0, 0]),
    );
    draw_filled_circle_mut(
        &mut image,
        (image_width - 10, image_height - 20 - 10),
        10,
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 20, image_height - 20 - 20).of_size(20, 10),
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 20, image_height - 20 - 20).of_size(10, 20),
        background_color,
    );

    draw_filled_rect_mut(&mut image, imageproc::rect::Rect::at(10, image_height-20).of_size(image_width as u32 -20,1), background_color);

    draw_text_mut(
        &mut image,
        Rgba([255, 255, 255, 255]),
        50,
        image_height / 2 - text_height / 2 - 10,
        scale,
        &font,
        destination,
    );
    let left_corner = imageproc::point::Point::new(image_width / 2 - 10, image_height - 20);
    let right_corner = imageproc::point::Point::new(image_width / 2 + 10, image_height - 20);
    let bottom_point = imageproc::point::Point::new(image_width / 2, image_height-3);
    draw_polygon_mut(
        &mut image,
        &vec![left_corner, right_corner, bottom_point],
        background_color,
    );
    draw_filled_circle_mut(&mut image, (image_width/2, image_height-3), r, Rgba([255,255,255,255]));
    let (width, height) = train_picture.dimensions();
    let scale = text_height as f64 / height as f64;
    let scaled_width = (scale * width as f64) as i32;
    let train_picture = image::imageops::resize(
        &train_picture,
        scaled_width as u32,
        text_height as u32,
        image::imageops::FilterType::CatmullRom,
    );
    image::imageops::replace(&mut image, &train_picture, 10, 10);
    (image_width, image_height, image.into_vec())
}


#[cfg(test)]
mod lib_test {
    use image::RgbaImage;

    use crate::load_icon;

    #[test]
    fn test_icon() {
        let icon = load_icon(1);
        let img = RgbaImage::from_raw(48, 48, icon).unwrap();
        img.save("ic_train.png");
    }
}