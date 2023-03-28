pub mod bindings;

pub use autocxx;
pub use autocxx::cxx;
pub use autocxx::prelude::*;

use bindings::external_types::{LoaderInterfaceWrapperImpl, Tiled2dMapLayerConfigWrapperImpl};
use cxx::CxxVector;
pub use ffi::*;

pub use bindings::{cxx_const_cast, cxx_shared_cast};

use autocxx_macro::extern_rust_function;

#[extern_rust_function]
pub fn log_rs(log_statement: String) {
    log::info!("{log_statement}");
}

unsafe impl Send for MapInterface {}
unsafe impl Sync for MapInterface {}
unsafe impl Send for MapReadyCallbackInterface {}
unsafe impl Sync for MapReadyCallbackInterface {}
unsafe impl Send for RectCoord {}
unsafe impl Sync for RectCoord {}

use crate::bindings::impls::*;
include_cpp! {
    #include "BoundingBox.h"
    #include "LineInfoInterface.h"
    #include "LineLayerInterface.h"
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
    #include "IconType.h"
    #include "SchedulerInterfaceStaticWrapper.h"
    #include "MapReadyCallbackInterface.h"
    #include "LayerReadyState.h"
    #include "LineInfoInterfaceWrapper.h"

    #include "Tiled2dMapLayerConfigWrapper.h"
    #include "ColorStateList.h"
    #include "LineCapType.h"
    #include "SizeType.h"
    #include "Vec2F.h"

    #include "IconInfoInterface.h"

    #include "IconLayerInterface.h"

    safety!(unsafe_ffi)
    generate!("IconType")
    generate!("BoundingBox")
    generate!("PolygonCoordBuilder")
    generate!("LayerReadyState")
    generate!("PolygonCoord")
    generate!("MapCallbackInterface")
    generate!("PolygonLayerInterface")
    generate!("LineLayerInterface")
    generate!("Vec2F")

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
    generate!("transform_icon_info_interface")
    generate!("MapScene")
    // subclass!("SchedulerInterfaceStaticWrapper", SchedulerInterfaceImpl)

    subclass!("TextureHolderInterface", TextureHolderInterfaceImpl)
    subclass!("IconInfoInterface", IconInfoInterfaceImpl)
    // subclass!("Tiled2dMapLayerConfigWrapper", Tiled2dMapLayerConfigWrapperImpl)
    subclass!("MapCallbackInterface", MapCallbackInterfaceImpl)
    generate!("IconLayerInterface")

    subclass!("MapReadyCallbackInterface", MapReadyCallbackInterfaceImpl)
    generate!("LineInfoInterfaceWrapper")
    generate!("LineInfoInterfaceWrapperBuilder")
    generate!("Tiled2dMapLayerConfigWrapper")
    generate!("SchedulerInterfaceStaticWrapper")

    generate!("ColorStateList")
    generate!("LineCapType")
    generate!("SizeType")
    generate!("LineStyle")
    generate!("make_default_dash")
}

// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

unsafe impl Send for TaskInterface {}
unsafe impl Sync for TaskInterface {}

pub trait TaskSpawner {
    fn spawn_blocking(&self, task: autocxx::cxx::SharedPtr<TaskInterface>);
}

pub struct DefaultSpawner {
    rt: tokio::runtime::Runtime,
}

impl TaskSpawner for DefaultSpawner {
    fn spawn_blocking(&self, task: autocxx::cxx::SharedPtr<TaskInterface>) {
        self.rt.spawn_blocking(move || {
            log::debug!("running: {}", get_id(task.clone()));
            run_task(task.clone());
            log::debug!("finished: {}", get_id(task));
        });
    }
}

type OptionalSender = Option<std::sync::mpsc::Sender<cxx::SharedPtr<TaskInterface>>>;
type OptionalSpawner = Option<Box<dyn TaskSpawner + Send + Sync>>;
type RuntimeType = (OptionalSender, OptionalSpawner);

pub struct SchedulerInterfaceRust {
    pub spawner: Box<dyn TaskSpawner>,
    pub channel: Sender<autocxx::cxx::SharedPtr<TaskInterface>>,
}
use std::sync::mpsc::{Receiver, Sender};

pub struct TaskReceiver(Receiver<autocxx::cxx::SharedPtr<TaskInterface>>);
#[cxx::bridge]
mod custom {
    extern "Rust" {
        type SchedulerInterfaceRust;
        type TaskReceiver;
        fn addTaskRust(self: &SchedulerInterfaceRust, task: SharedPtr<TaskInterface>);
        fn removeTaskRust(self: &SchedulerInterfaceRust, id: String);
        fn clearRust(self: &SchedulerInterfaceRust);
        fn resumeRust(self: &SchedulerInterfaceRust);
        fn pauseRust(self: &SchedulerInterfaceRust);
    }
    extern "C++" {
        include!("TaskInterface.h");
        type TaskInterface = super::TaskInterface;
    }
    impl Box<SchedulerInterfaceRust> {}
}

pub trait Tiled2dMapLayerConfigTrait {
    fn getCoordinateSystemIdentifier(&self) -> UniquePtr<cxx::CxxString>;
    fn getTileUrl(&self, x: i32, y: i32, t: i32, zoom: i32) -> UniquePtr<cxx::CxxString>;
    fn getZoomLevelInfos(&self) -> UniquePtr<CxxVector<Tiled2dMapZoomLevelInfo>>;
    fn getZoomInfo(&self) -> UniquePtr<Tiled2dMapZoomInfo>;
    fn getLayerName(&self) -> UniquePtr<cxx::CxxString>;
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

    }
    extern "C++" {
        include!("Tiled2dMapLayerConfigWrapper.h");
        include!("Tiled2dMapZoomLevelInfo.h");
        include!("Tiled2dMapZoomInfo.h");
        type Tiled2dMapZoomLevelInfo = super::Tiled2dMapZoomLevelInfo;
        type Tiled2dMapZoomInfo = super::Tiled2dMapZoomInfo;

    }
    impl Box<Tiled2dMapLayerConfigWrapperImpl> {}
}

impl SchedulerInterfaceRust {
    pub fn new() -> (Self, Receiver<autocxx::cxx::SharedPtr<TaskInterface>>) {
        let (sender, receiver) = std::sync::mpsc::channel();
        (
            Self {
                spawner: Box::new(DefaultSpawner {
                    rt: tokio::runtime::Builder::new_multi_thread()
                        .enable_all()
                        .max_blocking_threads(5)
                        .worker_threads(1)
                        .thread_keep_alive(std::time::Duration::from_secs(5))
                        .build()
                        .expect("Failed to build internal tasks runtime"),
                }),
                channel: sender,
            },
            receiver,
        )
    }
    fn addTaskRust(&self, task: autocxx::cxx::SharedPtr<TaskInterface>) {
        let t = task.clone();
        if !is_graphics(t.clone()) {
            self.spawner.spawn_blocking(task);
        } else {
            if self.channel.send(task).is_err() {
                log::error!("Could not submit task");
            }
        }
    }
    fn removeTaskRust(&self, id: String) {
        println!("remove {}", id);
        log::debug!("removeTask")
    }
    fn clearRust(&self) {
        log::debug!("clear")
    }
    fn pauseRust(&self) {
        log::debug!("pause")
    }
    fn resumeRust(&self) {
        log::debug!("resume")
    }
}

pub struct LayerInfoInterfaceRust {
    identifier: String,
    coordinates: UniquePtr<CxxVector<Coord>>,
    style: UniquePtr<LineStyle>,
}
pub fn new_line_info_wrapper(
    identifier: String,
    coordinates: UniquePtr<CxxVector<Coord>>,
    style: UniquePtr<LineStyle>,
) -> Box<LayerInfoInterfaceRust> {
    Box::new(LayerInfoInterfaceRust {
        identifier,
        coordinates,
        style,
    })
}
#[cxx::bridge]
mod LayerInfoInterfaceMod {
    extern "Rust" {
        type LayerInfoInterfaceRust;
        fn getIdentifier(&self) -> String;
        fn getCoordinates(&self) -> UniquePtr<CxxVector<Coord>>;
        fn getStyle(&self) -> UniquePtr<LineStyle>;
        fn new_line_info_wrapper(
            identifier: String,
            coordinates: UniquePtr<CxxVector<Coord>>,
            style: UniquePtr<LineStyle>,
        ) -> Box<LayerInfoInterfaceRust>;
    }
    extern "C++" {
        include!("LineStyle.h");
        include!("Coord.h");
        type LineStyle = super::LineStyle;
        type Coord = super::Coord;
    }
}

impl LayerInfoInterfaceRust {
    fn getIdentifier(&self) -> String {
        todo! {}
    }
    fn getCoordinates(&self) -> UniquePtr<CxxVector<Coord>> {
        todo! {}
    }
    fn getStyle(&self) -> UniquePtr<LineStyle> {
        todo! {}
    }
}

pub trait LoaderInterfaceTrait {
    fn loadTextureWrapper(
        &self,
        url: &cxx::CxxString,
        etag: cxx::UniquePtr<cxx::CxxString>,
    ) -> cxx::UniquePtr<TextureLoaderResult>;
    fn loadDataWrapper(
        &self,
        url: &cxx::CxxString,
        etag: cxx::UniquePtr<cxx::CxxString>,
    ) -> cxx::UniquePtr<DataLoaderResult>;
}

#[cxx::bridge]
mod LoaderInterfaceWrapperMod {
    extern "Rust" {
        type LoaderInterfaceWrapperImpl;
        fn loadTextureWrapper(
            &self,
            url: &CxxString,
            etag: UniquePtr<CxxString>,
        ) -> UniquePtr<TextureLoaderResult>;
        fn loadDataWrapper(
            &self,
            url: &CxxString,
            etag: UniquePtr<CxxString>,
        ) -> UniquePtr<DataLoaderResult>;

    }
    extern "C++" {
        include!("LoaderInterface.h");
        include!("TextureLoaderResult.h");
        include!("DataLoaderResult.h");
        type TextureLoaderResult = super::TextureLoaderResult;
        type DataLoaderResult = super::DataLoaderResult;

    }
    impl Box<LoaderInterfaceWrapperImpl> {}
}
