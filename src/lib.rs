use anyhow::bail;
use euclid::Size2D;

pub use gl;
pub use openmobilemaps_sys;
use std::{default::Default, time::Duration};
use surfman::{ContextAttributeFlags, ContextAttributes, GLVersion, SurfaceAccess, SurfaceType};

use openmobilemaps_sys::openmobilemaps_bindings::{
    autocxx::subclass::CppSubclass,
    bindings::impls::{MapCallbackInterfaceImpl, MapReadyCallbackInterfaceImpl},
    cxx::SharedPtr,
    *,
};

#[cfg(target_os = "linux")]
use surfman::platform::unix::generic::connection::Connection;
#[cfg(target_os = "linux")]
use surfman::platform::unix::generic::context::Context;
#[cfg(target_os = "linux")]
use surfman::platform::unix::generic::device::Device;

#[cfg(not(target_os = "linux"))]
use surfman::Connection;
#[cfg(not(target_os = "linux"))]
use surfman::Context;
#[cfg(not(target_os = "linux"))]
use surfman::Device;

pub type MapData = (
    std::sync::mpsc::Receiver<SharedPtr<TaskInterface>>,
    SharedPtr<MapInterface>,
    Option<std::sync::mpsc::Receiver<()>>,
    Option<UniquePtr<MapReadyCallbackInterface>>,
    Option<std::sync::mpsc::Receiver<LayerReadyState>>,
);

pub fn setup_opengl(view_port: (usize, usize)) -> anyhow::Result<(Device, Context)> {
    let Ok(connection) = Connection::new() else  {
        bail!("Failed to setup connection to display");
    };
    let Ok(adapter) = connection.create_adapter() else {
        bail!("Failed to find suitable adapter");
    };
    let Ok(mut device) = connection.create_device(&adapter) else {
        bail!("Failed to create device");
    };
    let context_attributes = ContextAttributes {
        version: GLVersion::new(4, 3),
        flags: ContextAttributeFlags::ALPHA
            | ContextAttributeFlags::STENCIL
            | ContextAttributeFlags::DEPTH,
    };
    let Ok(context_descriptor) = device
        .create_context_descriptor(&context_attributes)
        else {
            bail!("Failed to create context descriptor");
        };
    let Ok(mut context) = device.create_context(&context_descriptor, None) else {
        bail!("Failed to create context");
    };

    let Ok(surface) = device
        .create_surface(
            &context,
            SurfaceAccess::GPUOnly,
            SurfaceType::Generic {
                size: Size2D::new(view_port.0 as i32, view_port.1 as i32),
            },
        )
        else {
            let _ = device.destroy_context(&mut context);
            bail!("Failed to create drawing surface");
        };
    if device
        .bind_surface_to_context(&mut context, surface)
        .is_err()
    {
        let _ = device.destroy_context(&mut context);
        bail!("Could not bind surface to context");
    }

    if device.make_context_current(&context).is_err() {
        let _ = device.destroy_context(&mut context);
        bail!("Could not make context current");
    }
    log::debug!("Load GL pointers");
    gl::load_with(|s| device.get_proc_address(&context, s) as *const std::os::raw::c_void);

    let mut arrays = 0;
    log::debug!("Setup VBO");
    unsafe { gl::GenVertexArrays(1, &mut arrays) };
    unsafe { gl::BindVertexArray(arrays) };

    log::debug!("Clear flags");
    unsafe {
        gl::Disable(gl::CULL_FACE);
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::BLEND);
        // gl::Enable(gl::MULTISAMPLE);
        log::debug!("Bind framebuffer");
        let Ok(Some(surface_info)) = device.context_surface_info(&context) else {
            let _ = device.destroy_context(&mut context);
            bail!("Failed to get surface info");
        };
        gl::BindFramebuffer(gl::FRAMEBUFFER, surface_info.framebuffer_object);
        log::debug!("Set viewport");
        gl::Viewport(0, 0, view_port.0 as i32, view_port.1 as i32);
    }

    Ok((device, context))
}

pub fn setup_map(view_port: (usize, usize), with_invalidate: bool, with_ready: bool) -> anyhow::Result<MapData> {
    let coordsystem = CoordinateSystemFactory::getEpsg3857System();
    let map_config = MapConfig::new(coordsystem.within_unique_ptr()).within_unique_ptr();
    if map_config.is_null() {
        bail!("Could not create map config");
    }
    let (scheduler, task_receiver) = SchedulerInterfaceRust::new();
    let scheduler = Box::new(scheduler);

    let scheduler = unsafe { SchedulerInterfaceStaticWrapper::new1(Box::into_raw(scheduler) as _) }
        .within_unique_ptr();
    if scheduler.is_null() {
        bail!("Could not initialize schedulerinterface");
    }

    let scheduler = transform_unique(scheduler);
    let map_interface: SharedPtr<MapInterface> =
        MapInterface::createWithOpenGl(&map_config, &scheduler, 1.0);
    if map_interface.is_null() {
        bail!("Could not create map interface");
    }
    let invalidate_receiver = if with_invalidate {
        let (invalidate_sender, r) = std::sync::mpsc::channel();

        let mut callbacks = MapCallbackInterfaceImpl::default();
        callbacks.sender = Some(invalidate_sender);
        let callbacks = MapCallbackInterfaceImpl::new_cpp_owned(callbacks);
        if callbacks.is_null() {
            bail!("Could not initialize map callbacks");
        }
        let callback_interface =
            MapCallbackInterfaceImpl::as_MapCallbackInterface_unique_ptr(callbacks);

        pin_mut!(map_interface).setCallbackHandler(&to_map_callback_interface_shared_pointer(
            callback_interface,
        ));
        Some(r)
    } else {
        None
    };
    let (ready_state_interface, ready_state_receiver) = if with_ready {
        let (ready_state_sender, ready_state_receiver) = std::sync::mpsc::channel();

        let mut ready_state = MapReadyCallbackInterfaceImpl::default();
        ready_state.sender = Some(ready_state_sender);
        let ready_state = MapReadyCallbackInterfaceImpl::new_cpp_owned(ready_state);
        if ready_state.is_null() {
            bail!("Callback interface was unexpectedly null");
        }
        (
            Some(
                MapReadyCallbackInterfaceImpl::as_MapReadyCallbackInterface_unique_ptr(ready_state),
            ),
            Some(ready_state_receiver),
        )
    } else {
        (None, None)
    };
    pin_mut!(map_interface).setViewportSize(&Vec2I::new(view_port.0 as i32, view_port.1 as i32).within_unique_ptr());
    Ok((
        task_receiver,
        map_interface,
        invalidate_receiver,
        ready_state_interface,
        ready_state_receiver,
    ))
}

pub fn draw_ready_frame(
    view_port: (usize, usize),
    map_interface: SharedPtr<openmobilemaps_sys::openmobilemaps_bindings::MapInterface>,
    rx: std::sync::mpsc::Receiver<
        SharedPtr<openmobilemaps_sys::openmobilemaps_bindings::TaskInterface>,
    >,
    bounds: UniquePtr<RectCoord>,
    ready_state_interface: SharedPtr<
        openmobilemaps_sys::openmobilemaps_bindings::MapReadyCallbackInterface,
    >,
    display: &Device,
    context: &mut Context,
    ready_state_receiver: std::sync::mpsc::Receiver<LayerReadyState>,
) -> Vec<u8> {
    pin_mut!(map_interface).setViewportSize(&Vec2I::new(view_port.0 as i32, view_port.1 as i32).within_unique_ptr());
    let map_interface2 = map_interface.clone();
    std::thread::spawn(move || {
        let map_interface = map_interface2;
        pin_mut!(map_interface).drawReadyFrame(&bounds, 10.0, &ready_state_interface);
    });

    let mut buffer = vec![0u8; view_port.0 * view_port.1 * 4];

    let _ = display.make_context_current(context);

    loop {
        pin_mut!(map_interface).drawFrame();
        while let Ok(task) = rx.try_recv() {
            run_task(task);
        }

        if let Ok(state) = ready_state_receiver.try_recv() {
            if state == LayerReadyState::READY {
                pin_mut!(map_interface).drawFrame();
                pin_mut!(map_interface).pause();
                while let Ok(task) = rx.try_recv() {
                    run_task(task);
                }

                unsafe {
                    gl::Finish();
                    gl::ReadPixels(
                        0,
                        0,
                        view_port.0 as i32,
                        view_port.1 as i32,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        buffer.as_mut_ptr() as _,
                    );
                }
                break;
            }
        }

        std::thread::yield_now();
        std::thread::sleep(Duration::from_millis(10));
    }

    buffer
}
