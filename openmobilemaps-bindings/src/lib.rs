pub mod bindings;
pub use autocxx::prelude::*;

use bindings::external_types::Tiled2dMapLayerConfigWrapperImpl;
pub use ffi::*;

pub use bindings::{cxx_const_cast, cxx_shared_cast};

use autocxx_macro::{extern_rust_function, subclass};

#[extern_rust_function]
pub fn log_rs(log_statement: String) {
    log::info!("{log_statement}");
}
use crate::bindings::impls::*;
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

// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use autocxx::cxx;
use cxx::SharedPtr;

use crate::ffi::*;

unsafe impl Send for TaskInterface {}
unsafe impl Sync for TaskInterface {}

pub mod SchedulerInterfaceImplPool {
    use super::TaskInterface;

    lazy_static::lazy_static! {
       pub static ref STATIC_RUNTIME_POOL : std::sync::Mutex< (Option<std::sync::mpsc::Sender<cxx::SharedPtr<TaskInterface>>>, tokio::runtime::Runtime)> = {
            std::sync::Mutex::new((None, tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .max_blocking_threads(512)
            .thread_keep_alive(std::time::Duration::from_secs(30))
            .build()
            .unwrap()))
        };
    }
}

pub struct SchedulerInterfaceRust {}
pub fn new_task_interface() -> Box<SchedulerInterfaceRust> {
    Box::new(SchedulerInterfaceRust {})
}
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
        type TaskInterface = super::TaskInterface;
    }
}

pub fn new_layer_config_inner_wrapper() -> Box<Tiled2dMapLayerConfigWrapperImpl> {
    Box::new(Tiled2dMapLayerConfigWrapperImpl)
}

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
        type Tiled2dMapZoomLevelInfo = super::Tiled2dMapZoomLevelInfo;
        type Tiled2dMapZoomInfo = super::Tiled2dMapZoomInfo;
    }
}

impl SchedulerInterfaceRust {
    fn addTaskRust(&self, task: SharedPtr<TaskInterface>) {
        let t = task.clone();
        if !is_graphics(t.clone()) {
            let spawner = SchedulerInterfaceImplPool::STATIC_RUNTIME_POOL
                .lock()
                .unwrap();
            spawner.1.spawn_blocking(move || {
                // println!("running: {}", get_id(t.clone()));
                run_task(t.clone());
                // println!("finished: {}", get_id(t.clone()));
            });
        } else if let Ok(sender) = SchedulerInterfaceImplPool::STATIC_RUNTIME_POOL.lock() {
            if let Some(sender) = sender.0.as_ref() {
                let _ = sender.send(t);
            }
        }
    }
    fn removeTaskRust(&self, id: String) {
        println!("removeTask")
    }
    fn clearRust(&self) {
        println!("clear")
    }
    fn pauseRust(&self) {
        println!("pause")
    }
    fn resumeRust(&self) {
        println!("resume")
    }
}
