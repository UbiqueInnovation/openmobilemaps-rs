use euclid::Size2D;
use glium::glutin::{
    dpi::{PhysicalSize, Size},
    ContextBuilder,
};

use glium::backend::glutin::headless::Headless;
use image::{GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{
    draw_filled_circle_mut, draw_filled_rect_mut, draw_polygon_mut, draw_text_mut, text_size,
};
use rusttype::{Font, Scale};
use surfman::{
    Connection, ContextAttributeFlags, ContextAttributes, GLVersion, SurfaceAccess, SurfaceType,
};

use std::{
    default::Default,
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

fn main() {
    let start = Instant::now();
    println!("Setup opengl");
    let (mut display, mut context) = setup_opengl();

    println!("Create map");
    let (rx, map_interface, invalidate_receiver, ready_state_interface, ready_state_receiver) =
        setup_map();
    println!("Add raster layer");
    let (loader_interface_ptr, raster_layer) = create_raster_layer();

    let line_layer = LineLayerInterface::create();

    let color_red = Color::new(1.0, 0.3, 0.34, 1.0).within_box();
    let color_red_outer = [
        (1.0 * 255.0) as u8,
        (0.3 * 255.0) as u8,
        (0.34 * 255.0) as u8,
        150,
    ];
    let color_red_inner = [
        (1.0 * 255.0) as u8,
        (0.3 * 255.0) as u8,
        (0.34 * 255.0) as u8,
        255,
    ];

    println!("Add polyline");
    let (points_a, connection_a_interface) = new_poly_line("eqeaHqiim@vBcJrBwIXkA\\uA^cBJo@Jm@d@qBR{@Hc@Le@n@{BLm@VsAXeBPiAPoANaBFqB@cBEeBMkBK_AMcAQw@o@eCk@qBEO_@oAWeAUcAWyAMs@c@}CMsAImAGmAGcDAuDBcBDaCD}@HyADu@Hu@RcBb@wCj@yCv@gDzCgKlAcEbBsGXkArBeJfAqEpAuFVmAZgB|@{Fb@cC\\_BNo@xAmGdFiTdAiEtAsFhEqQzDmQhByHPu@jAgFvBeJhByHvCcMf@wBhE{QDO|C{Mn@oCr@iDr@eDxAuFPq@n@qBp@qBt@sBx@qB|@oBz@}An@gAZi@`BaCbByB`BaB`B_BzAqA|AeAxAaAfFsDdFiEdMsMnEqGlEqHpFaLlFwLxS{d@``AuuBvH{RpKkYhNi_@lE}JtEiJnG{KrHkLb@m@bDiErA_BnAaBZa@pBiCdByB|@mA~AeCbBoCz@sAhAiBlAqBv@sAhBaDfB_D??xB_ExAkCj@_AbAgBf@aAZi@r@oAp@kAh@cAnByDtA}CvEuLfIuTp@gBtBaGz@aC|@}BhUon@fJoWlAaEd@cB^aBj@yCXqBLoAN_BJ_BJmCBsB?wACyEAc@@{D?sELuDf@_Kr@kKZ_EPwBZwE`GiaAf@sIvAqUvGecAf@qHl@mH~@mFpAoElB_GfAuDd@yBDSn@qDn@yDhF_]r@yF~@eIBS|BmT`@aENcE?_CGiCMyBCYCO]aCMs@WgAQm@[iAUm@a@}@[m@[g@i@{@e@m@q@{@sEyEaC{DmA{Cw@mDYkBEYMuAKkAIaC?mBJiFJ_DHyE?kBGoBSaEOaAWwBK_Ae@yDo@cF}AiMuAiLKw@g@sDOqAS_Ba@iDUmCO_EBeETyDd@iDj@iCRu@b@qA^w@d@_A`AaBV_@^c@hAgAj@c@`Aw@jHoF~BiBtBsBpBgCjBuCrCiGpAwDjAeF`DkR`@gB\\qA^oAv@cCl@uAzA_D~@yA`GiGzR{QX[zByBhAwA`@k@d@o@hBsDl@sAjAgC^_AtAkDjB}D~BaDlBoBdGmFzIkIvBeB`CmArAe@bBi@r@U|Bu@jr@sRjrEalAfoAw\\fuAi\\~IuB`FsAhKmCzHqBxCw@n@Qx@SdB]jAK|@Az@Bz@DpBb@lA^|Ap@`B~@~AnAz@v@v@z@v@~@r@dAp@hAj@fAvA|C`E|JBHTl@j@bB`@bAbAdCr@pAf@v@l@t@l@n@jA|@TNZNXNZLd@Tb@Px@RdAVoB[g@O[C[EZNbAf@dAPn@NlATt@JjEb@??", &color_red);

    let color_green = Color::new(0.39, 0.45, 0.65, 1.0).within_box();
    let color_blue_outer = [
        (0.39 * 255.0) as u8,
        (0.45 * 255.0) as u8,
        (0.65 * 255.0) as u8,
        150,
    ];
    let color_blue_inner = [
        (0.39 * 255.0) as u8,
        (0.45 * 255.0) as u8,
        (0.65 * 255.0) as u8,
        255,
    ];

    let (points_b, connection_b_interface) = new_poly_line("osd`Hyzbs@ABsEnWObAKr@UfCG\\EZWvAId@Oj@Ob@O\\Wd@a@|@Ql@Qv@Kx@Q~AMvAWhDIz@It@Gh@Q`AMj@Sz@Wp@Wp@mAnCiAfCy@fCw@bCGRMd@UdAsBlLOt@Of@Md@Kd@CLIj@Ip@KrAIj@Ml@Mp@U`Ag@dCu@nE[fBc@jC}@pGg@`Dk@~C_@hBc@fBu@rCa@tA{@jCKXw@vC_@zASbAUlAQ`Ac@zCG\\G^Mh@Mj@Sx@GTMp@Mh@K`@GXCPKv@CJCl@Ah@EvA?VC^C`@EZEX}@pFS~@ZiAT{@Pq@Jg@Jg@yC~QQfAOfAQzAOxAKrAKrAKrBI`BIxBU`FKhBGfAKrAMpAOlAM~@M`AqC`P??aAtFsAfJKn@]xBuDlUsA`IkChOkBlKqQvcAcFdW[jBMx@Kx@MbAIdAGz@G|@EfAChACbA?dA@t@?t@Bz@B`A^jMJrCNfFDvBB`C@rAH~GDhDLrEh@jPj@xLFfARfEJfC`AdVRpGRxFLrDPpDPhDPfDFnB@vB?rAAvBItBMbCSbCU`Cm@vGm@xGw@hJUzCg@dHShCqCj\\[`DQ|AOx@O|@Qt@Qv@[hAy@zBYr@k@dA[j@a@l@a@h@g@h@a@`@c@`@a@Za@Xe@VYPeAb@{@^aLdFiAh@aAd@kAn@yGtD{A`AeAt@}AnAGFoAjA_A`AaCxCkA`BoCfEmi@ry@}GvJq@dAc@p@e@v@w@zAwA~Cq@bBc@rA_@nA_@nAg@nByBbJk@|Bm@|BeBdGsA~E}c@ffBqAbFsDbNiAvEcBdHeAjE}@nDyAxFeAfEqBxHK`@oFtSeA`Ew@|CW~@U`AW`ASbAKd@Kf@Kf@Kf@y@pGg@xGQhEGlE@zDF`CHxBNlCRdCh@dGD^`CzWjF|k@|j@biGZjDRzBb@dEh@|EzA~MtAxL`@bEHx@JdBLxCHtCBhDAxCC~ACtBKhISxPCzAGbGBtCDxCJxCPtCRvBPlBVhBVbBVnA^dB@FzAdGLZp@pBbAdC`AnBlA|BbA~Ab@j@zAnBvAzAhC|BZVpBzAhCnB|ArAh@f@n@n@l@p@v@z@JJr@z@n@z@PRr@bA|A~Bz@vALThAxB~@lB|@pBx@rBt@pBr@vBr@dCf@bBdAvEj@jC`AxEJj@^jBZ|APx@dAbF~@|E|ClPlH``@bAjFXxAf@jCl@pCx@rCXv@rAfDf@dAtAhChAnBPZDHbCnEb@z@tB~D|BrEZj@lBzDdAjBp@bAr@|@r@z@jAlAx@n@`Ap@l@`@RJpAp@nB~@dDzArBz@jAd@xAj@tAh@~Aj@bCx@|An@h@Vh@XjBnA`BxAdAjAt@`Az@pA`@r@|@hBv@hBp@rBn@~Bb@tBZjBJj@BNPlBHt@LlApArOlBzTtF~p@vApPPlBRhCFp@z@nKJpB@XD~A@bCEdBEzAOhCObBOlAG`@WvAUlAg@zBwBdJ{AdH{AtHwDxRq@bDk@jCABgDrO}Gf[oGtYq@zCs@tCm@vBq@rBgArCoAlCeAjBeAdB_EtF_CpDgAjB_@x@eA`Cu@lBs@tBo@|Bk@~B_@hB[bBa@pCUpBSxBMhBIfBEdAG`CAlAAxCDvCDxADrAV`FN~BH|@RhCp@~Gf@lEn@tE|AdLhAhKHn@b@lEl@fH`AvL?DJpAdZdyD|AfSNnBPrBXhDl@bIv@|K|@pOZjEz@~KL`Bj@dHdBfUpCt]R`Ch@dFTbBTdBp@pEBLrBjMf@jCLl@Z`Bd@hCdA`G`A|Fh@vCh@rCj@hCbAtE`AvEj@`Df@`Dj@fEdBtOr@zFT~A\\nCd@hDp@jE^bCxFl^x@~FVxAx@nE~@rEv@bDpAjF`ArDhB`Gr@rBr@lBh@nA~@vBhA~BrAdCjAnBxAzBrAfB`BpBvAzAzAzAhAbA|AjApAbAnBpArBhArAn@vB|@fBn@tA`@jJlCtAf@dBt@pAj@pAr@tAv@|AfA|AhAjB|AvBtBvAzArA`BfAtAxAtBdBvCvAfCfAxBhAnCrAdDbA|ChAtDhAnEbA|EdAjGh@dEh@rEZpC`@rD`@tCp@nEn@dDt@jD~@pDr@rCNh@bIlZ`DnLp@fCvAnFhDbLp@zBZjAdBvG~@nD~CpLZjA`Ldb@hBzGbC|I|@`D`@fBb@~BZpBTrBZxF@rGWzF]nD[vBc@vBk@~Bq@tBs@lBq@fBmBzEmA|Ck@dBm@nB]tA]~A[dBUhB[`DYdE}GpiA[dEm@|Fg@fDi@`Dw@pDc@fBK\\e@`Bk@dBgAvCmA|CcBpD_DdGk@pASb@e@jAa@hAk@xBU~@_@tB[|CIdAClACtA@|AF~ALlBNvAl@|CVdA^pAh@~Ad@pAtLpYVn@b@x@f@z@f@r@j@n@d@f@l@d@tAt@z@Zd@JXNVJZNbAf@dAPn@NlATt@JjEb@??", &color_green);

    println!("Add polylines");
    pin_mut!(line_layer).add(&connection_a_interface);
    pin_mut!(line_layer).add(&connection_b_interface);

    let line_layer = pin_mut!(line_layer).asLayerInterface();

    pin_mut!(map_interface).addLayer(&raster_layer);
    pin_mut!(map_interface).addLayer(&line_layer);

    println!("Setup camera");
    let camera = pin_mut!(map_interface).getCamera();
    pin_mut!(camera).setMaxZoom(0.0);
    pin_mut!(camera).setMinZoom(f64::MAX);

    let icon_layer = IconLayerInterface::create();

    let mut the_icon = IconInfoInterfaceImpl::default();
    the_icon.texture_data = get_start_point("1", color_red_outer, color_red_inner, 80, 80);
    the_icon.image_width = 80;
    the_icon.image_height = 80;
    the_icon.anchor = (0.5, 0.5);
    the_icon.coordinate = (
        CoordinateSystemIdentifiers::EPSG4326()
            .to_string_lossy()
            .as_ref()
            .to_string(),
        points_a[0].0,
        points_a[0].1,
    );
    let the_icon = IconInfoInterfaceImpl::new_cpp_owned(the_icon);
    let the_icon = IconInfoInterfaceImpl::as_IconInfoInterface_unique_ptr(the_icon);
    let the_icon = transform_icon_info_interface(the_icon);
    pin_mut!(icon_layer).add(&the_icon);

    let mut the_icon = IconInfoInterfaceImpl::default();
    the_icon.texture_data = get_start_point("2", color_blue_outer, color_blue_inner, 80, 80);
    the_icon.image_width = 80;
    the_icon.image_height = 80;
    the_icon.anchor = (0.5, 0.5);
    the_icon.coordinate = (
        CoordinateSystemIdentifiers::EPSG4326()
            .to_string_lossy()
            .as_ref()
            .to_string(),
        points_b[0].0,
        points_b[0].1,
    );
    let the_icon = IconInfoInterfaceImpl::new_cpp_owned(the_icon);
    let the_icon = IconInfoInterfaceImpl::as_IconInfoInterface_unique_ptr(the_icon);
    let the_icon = transform_icon_info_interface(the_icon);
    pin_mut!(icon_layer).add(&the_icon);

    let mut the_icon = IconInfoInterfaceImpl::default();
    let (icon_width, icon_height, texture_data) = get_destination_box("Olten");
    the_icon.texture_data = texture_data;
    the_icon.image_width = icon_width as usize;
    the_icon.image_height = icon_height as usize;
    the_icon.anchor = (0.5, 1.0);
    the_icon.coordinate = (
        CoordinateSystemIdentifiers::EPSG4326()
            .to_string_lossy()
            .as_ref()
            .to_string(),
        7.9078318,
        47.3520027,
    );
    let the_icon = IconInfoInterfaceImpl::new_cpp_owned(the_icon);
    let the_icon = IconInfoInterfaceImpl::as_IconInfoInterface_unique_ptr(the_icon);
    let the_icon = transform_icon_info_interface(the_icon);
    pin_mut!(icon_layer).add(&the_icon);

    let icon_layer = pin_mut!(icon_layer).asLayerInterface();
    pin_mut!(map_interface).insertLayerAbove(&icon_layer, &line_layer);

    println!("added icon layer");

    let center_coord = Coord::new(
        CoordinateSystemIdentifiers::EPSG4326(),
        7.9078318,
        47.3520027,
        0.0,
    )
    .within_unique_ptr();

    pin_mut!(map_interface).resume();
    pin_mut!(camera).setPaddingBottom(178.0);
    pin_mut!(camera).setPaddingLeft(100.0);
    pin_mut!(camera).setPaddingRight(100.0);
    pin_mut!(camera).setPaddingTop(100.0);

    pin_mut!(map_interface).setBackgroundColor(&Color::new(0.0, 0.0, 0.0, 1.0).within_unique_ptr());

    let b = BoundingBox::new1(&CoordinateSystemIdentifiers::EPSG4326()).within_unique_ptr();
    for &(x, y) in points_a.iter().chain(points_b.iter()) {
        pin_mut!(b).addPoint(x, y, 0.0);
    }

    let bounds = pin_mut!(b).asRectCoord().within_unique_ptr();

    let map_interface2 = map_interface.clone();
    let ready_state_interface = transform_ready_state(ready_state_interface);

    std::thread::spawn(move || {
        let map_interface = map_interface2;
        pin_mut!(map_interface).drawReadyFrame(&bounds, 10.0, &ready_state_interface);
    });

    //

    println!("Start rendering loop");
    display.make_context_current(&context);
    loop {
        // let frame = display.draw();
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
        // frame.finish().unwrap();

        if let Ok(state) = ready_state_receiver.recv_timeout(Duration::from_millis(1)) {
            if state == LayerReadyState::READY {
                break;
            }
        }
    }

    println!("finishing frame");
    pin_mut!(map_interface).drawFrame();
    pin_mut!(map_interface).drawFrame();
    pin_mut!(map_interface).drawFrame();
    unsafe {
        gl::Flush();
        gl::Finish();
    };
    std::thread::sleep(Duration::from_millis(5));
    let mut buffer = vec![0u8; 1200 * 630 * 4];
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
    let mut max_vertex_attribs = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs) };
    println!("GL_MAX_VERTEX_ATTRIBS {}", max_vertex_attribs);

    display.destroy_context(&mut context);

    println!("{} {} {} {}", buffer[0], buffer[1], buffer[2], buffer[3]);

    // println!("{}", display.get_opengl_renderer_string());

    // let image: glium::texture::RawImage2d<'_, u8> = display.read_front_buffer().unwrap();

    let image = image::ImageBuffer::from_raw(1200, 630, buffer).unwrap();

    let image = image::DynamicImage::ImageRgba8(image).flipv();
    let mut image = image.resize_exact(1200, 630, image::imageops::FilterType::Lanczos3);

    let bottomstuff = image::open("./bottomstuff.jpeg").unwrap();
    let bottomstuff = bottomstuff.resize_exact(1200, 78, image::imageops::FilterType::Lanczos3);
    let bottomsheet = image.dimensions().1 as i64 - 78;
    image::imageops::replace(&mut image, &bottomstuff, 0, bottomsheet);
    image
        .save("glium-example-screenshot_framebuffer.png")
        .unwrap();
    let end = Instant::now();
    println!("Took {}ms", (end - start).as_millis());
}

fn setup_opengl() -> (surfman::Device, surfman::Context) {
    // let event_loop = glium::glutin::event_loop::EventLoop::new();
    // let wb = glium::glutin::window::WindowBuilder::new()
    //     .with_inner_size(Size::Physical(PhysicalSize {
    //         width: 1200,
    //         height: 630,
    //     }))
    //     .with_visible(true);
    // let cb = ContextBuilder::new()
    //     // .with_double_buffer(Some(true))
    //     // .with_multisampling(256)
    //     // .with_depth_buffer(8)
    //     // .with_stencil_buffer(8)
    //     .with_pixel_format(24, 8);
    // // .with_vsync(true);
    // let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    // let event_loop = glium::glutin::event_loop::EventLoop::new();
    // // let cb = ContextBuilder::new();
    // let size = PhysicalSize {
    //     width: 1200,
    //     height: 630,
    // };
    // // let context = cb.build_headless(&event_loop, size).unwrap();
    // // let context = unsafe { context.treat_as_current() };
    // // let display = glium::backend::glutin::headless::Headless::new(context).unwrap();

    // let ctx = glium::glutin::ContextBuilder::new()
    //     .build_headless(&event_loop, size)
    //     .unwrap();
    // let context = unsafe { ctx.treat_as_current() };

    // Unlike the other example above, nobody created a context for your window, so you need to create one.
    //     glium::glutin::HeadlessRendererBuilder::new(1200,630);
    // let event_loop = glium::glutin::event_loop::EventLoop::new();
    // let cb = glium::glutin::ContextBuilder::new().with_gl_profile(glium::glutin::GlProfile::Core).with_gl(glium::glutin::GlRequest::Latest);
    // let size = PhysicalSize {
    //     width: 1200,
    //     height: 630,
    // };

    // let context = cb.build_headless(&event_loop, size).unwrap();
    // let context = unsafe { context.treat_as_current() };
    // let display = glium::backend::glutin::headless::Headless::new(context).unwrap();
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

fn new_poly_line(
    connection_str: &str,
    color: &Color,
) -> (Vec<(f64, f64)>, SharedPtr<LineInfoInterface>) {
    let line_layer_info_interface = LineInfoInterfaceWrapperBuilder::new().within_unique_ptr();
    println!("decode polyline");
    let poly_line = polyline::decode_polyline(connection_str, 5).unwrap();
    println!("add points");
    let mut coords = vec![];
    for coord in poly_line.points() {
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

    println!("add style");
    let line_style = LineStyle::new(
        ColorStateList::new(color, color).within_box(),
        ColorStateList::new(color, color).within_box(),
        1.0,
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

struct ZoomInfo;

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

fn create_raster_layer() -> (SharedPtr<LoaderInterfaceImpl>, SharedPtr<LayerInterface>) {
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

fn setup_map() -> (
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

fn get_start_point(
    number: &str,
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
        Rgba([255, 255, 255, 255]),
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

fn get_destination_box(destination: &str) -> (i32, i32, Vec<u8>) {
    let font = Vec::from(include_bytes!("../AvertaStd-Bold.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let height = 40;
    let scale = Scale {
        x: height as f32,
        y: height as f32,
    };

    let train_picture = image::open("train.png").unwrap();
    let (text_width, text_height) = text_size(scale, &font, destination);
    let background_color = Rgba([64_u8, 72_u8, 137_u8, 250_u8]);
    let image_width = text_width + 20 + 50;
    let image_height = text_height + 20 + 20;
    println!("{} / {}", image_width, image_height);
    let mut image = RgbaImage::new(image_width as u32, image_height as u32);
    let full_rect =
        imageproc::rect::Rect::at(0, 0).of_size(image_width as u32, image_height as u32 - 19);
    draw_filled_rect_mut(&mut image, full_rect, background_color);

    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(0, 0).of_size(10, 10),
        Rgba([0, 0, 0, 0]),
    );
    draw_filled_circle_mut(&mut image, (4, 4), 5, background_color);
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(5, 0).of_size(10, 10),
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(0, 5).of_size(10, 10),
        background_color,
    );

    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 10, 0).of_size(10, 10),
        Rgba([0, 0, 0, 0]),
    );
    draw_filled_circle_mut(&mut image, (image_width - 5, 5), 5, background_color);
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 10, 0).of_size(5, 5),
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 10, 5).of_size(10, 5),
        background_color,
    );

    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(0, image_height - 20 - 10).of_size(10, 10),
        Rgba([0, 0, 0, 0]),
    );
    draw_filled_circle_mut(&mut image, (5, image_height - 20 - 5), 5, background_color);
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(0, image_height - 20 - 10).of_size(5, 5),
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(5, image_height - 20 - 10).of_size(10, 10),
        background_color,
    );

    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 10, image_height - 20 - 10).of_size(10, 10),
        Rgba([0, 0, 0, 0]),
    );
    draw_filled_circle_mut(
        &mut image,
        (image_width - 5, image_height - 20 - 5),
        5,
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 10, image_height - 20 - 10).of_size(10, 5),
        background_color,
    );
    draw_filled_rect_mut(
        &mut image,
        imageproc::rect::Rect::at(image_width - 10, image_height - 20 - 10).of_size(5, 10),
        background_color,
    );

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
    let bottom_point = imageproc::point::Point::new(image_width / 2, image_height);
    draw_polygon_mut(
        &mut image,
        &vec![left_corner, right_corner, bottom_point],
        background_color,
    );
    let (width, height) = train_picture.dimensions();
    let scale = text_height as f64 / height as f64;
    let scaled_width = (scale * width as f64) as i32;
    let train_picture = image::imageops::resize(
        &train_picture,
        scaled_width as u32,
        text_height as u32,
        image::imageops::FilterType::Lanczos3,
    );
    image::imageops::replace(&mut image, &train_picture, 10, 10);
    (image_width, image_height, image.into_vec())
}

#[cfg(test)]
mod image_tests {
    use image::RgbaImage;

    use crate::{get_destination_box, get_start_point};
    #[test]
    fn test_destination_box() {
        let (width, height, image_data) = get_destination_box("Hallo Welt");
        let image = RgbaImage::from_raw(width as u32, height as u32, image_data).unwrap();
        image.save("destination_box.png");
    }
    #[test]
    fn test_circle() {
        let color_outer = [
            (1.0 * 255.0) as u8,
            (0.3 * 255.0) as u8,
            (0.34 * 255.0) as u8,
            50,
        ];
        let color_inner = [
            (1.0 * 255.0) as u8,
            (0.3 * 255.0) as u8,
            (0.34 * 255.0) as u8,
            255,
        ];
        let start_point = RgbaImage::from_raw(
            800,
            800,
            get_start_point("1", color_outer, color_inner, 80, 80),
        )
        .unwrap();

        start_point.save("test_output.png");
    }
}
