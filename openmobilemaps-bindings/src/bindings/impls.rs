// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;
use std::sync::mpsc::Sender;

use autocxx::{subclass::*, WithinUniquePtr};
use autocxx_macro::subclass;
use cxx::SharedPtr;
use image::GenericImageView;

use crate::ffi;
use crate::ffi::*;
use crate::LoaderInterfaceTrait;

#[subclass(superclass("MapReadyCallbackInterface"))]
#[derive(Default)]
pub struct MapReadyCallbackInterfaceImpl {
    pub sender: Option<Sender<LayerReadyState>>,
}

impl MapReadyCallbackInterface_methods for MapReadyCallbackInterfaceImpl {
    fn stateDidUpdate(&mut self, state: LayerReadyState) {
        if let Some(sender) = self.sender.as_ref() {
            if state == LayerReadyState::READY {
                let _ = sender.send(LayerReadyState::READY);
            }
        }
    }
}

#[subclass(superclass("IconInfoInterface"))]
#[derive(Default)]
pub struct IconInfoInterfaceImpl {
    pub texture_data: Vec<u8>,
    pub image_width: usize,
    pub image_height: usize,
    pub coordinate: (String, f64, f64),
    pub anchor: (f64, f64),
}

impl IconInfoInterfaceImpl {
    pub fn as_shared_ptr(self) -> SharedPtr<IconInfoInterface> {
        let the_icon = IconInfoInterfaceImpl::new_cpp_owned(self);
        let the_icon = IconInfoInterfaceImpl::as_IconInfoInterface_unique_ptr(the_icon);
        transform_icon_info_interface(the_icon)
    }
}

impl IconInfoInterface_methods for IconInfoInterfaceImpl {
    fn getIdentifier(&mut self) -> cxx::UniquePtr<cxx::CxxString> {
        make_string("test")
    }

    fn getTexture(&mut self) -> cxx::SharedPtr<crate::TextureHolderInterface> {
        let mut interface = TextureHolderInterfaceImpl {
            image_width: self.image_width,
            image_height: self.image_height,
            texture_data: self.texture_data.clone(),
            ..Default::default()
        };
        let mut load_result = TextureHolderInterfaceImpl::new_cpp_owned(interface);
        let tex_holder_iface =
            TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);

        transform_texture_holder_interface(tex_holder_iface)
    }

    fn setCoordinate(&mut self, coord: &ffi::Coord) {}

    fn getCoordinate(&mut self) -> cxx::UniquePtr<ffi::Coord> {
        Coord::new(
            make_string(&self.coordinate.0),
            self.coordinate.1,
            self.coordinate.2,
            0.0,
        )
        .within_unique_ptr()
    }

    fn setIconSize(&mut self, size: &ffi::Vec2F) {}

    fn getIconSize(&mut self) -> crate::UniquePtr<ffi::Vec2F> {
        ffi::Vec2F::new(self.image_width as f32, self.image_height as f32).within_unique_ptr()
    }

    fn setType(&mut self, scaleType: ffi::IconType) {}

    fn getType(&mut self) -> ffi::IconType {
        ffi::IconType::INVARIANT
    }

    fn getIconAnchor(&mut self) -> crate::UniquePtr<ffi::Vec2F> {
        ffi::Vec2F::new(self.anchor.0 as f32, self.anchor.1 as f32).within_unique_ptr()
    }
}

#[subclass(superclass("MapCallbackInterface"))]
#[derive(Default)]
pub struct MapCallbackInterfaceImpl {
    pub sender: Option<Sender<()>>,
}

impl MapCallbackInterface_methods for MapCallbackInterfaceImpl {
    fn invalidate(&mut self) {
        if let Some(sender) = self.sender.as_ref() {
            sender.send(());
        }
    }
}

#[subclass(superclass("TextureHolderInterface"))]
#[derive(Default)]
pub struct TextureHolderInterfaceImpl {
    image_width: usize,
    image_height: usize,
    texture_data: Vec<u8>,
    usage_counter: usize,
    id: u32,
    attached: bool,
}

impl TextureHolderInterface_methods for TextureHolderInterfaceImpl {
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
            unsafe {
                let internal_format = gl::RGBA;
                log::debug!("load texture with gl");
                log::debug!(
                    "datalength: {} {} {} {}",
                    self.texture_data[0],
                    self.texture_data[1],
                    self.texture_data[2],
                    self.texture_data[3]
                );
                unsafe {
                    gl::GenTextures(1, &mut self.id);

                    gl::BindTexture(gl::TEXTURE_2D, self.id);

                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
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
                    // gl::GenerateMipmap(gl::TEXTURE_2D);
                }
            };
            self.attached = true;
        }
        self.usage_counter += 1;
        self.id as i32
    }

    fn clearFromGraphics(&mut self) {
        log::debug!("Clear texture");
        if self.usage_counter == 0 {
            self.attached = false;
            unsafe { gl::DeleteTextures(1, &mut self.id) };
        } else {
            self.usage_counter -= 1;
        }
    }
}

pub struct DefaultLoaderInterface(pub bool);
impl Drop for DefaultLoaderInterface {
    fn drop(&mut self) {
        log::debug!("Drop default loader interface");
    }
}
impl LoaderInterfaceTrait for DefaultLoaderInterface {
    fn loadTextureWrapper(
        &self,
        url: &cxx::CxxString,
        etag: cxx::UniquePtr<cxx::CxxString>,
    ) -> cxx::UniquePtr<TextureLoaderResult> {
        let u = url::Url::parse(url.to_str().unwrap()).unwrap();
        let p = u.path();
        let path = std::path::PathBuf::from_str(&format!("tiles/{p}")).unwrap();
        let mut databytes = vec![];
        if !path.exists() {
            let data = match ureq::get(url.to_str().unwrap()).call() {
                Ok(data) => data,
                Err(e) => {
                    let load_result = TextureHolderInterfaceImpl::default_cpp_owned();
                    let tex_holder_iface =
                        TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(
                            load_result,
                        );
                    let tex_holder_iface = transform_texture_holder_interface(tex_holder_iface);
                    if self.0 {
                        return make_loader_result(tex_holder_iface, LoaderStatus::OK);
                    } else {
                        match e.into_response() {
                            Some(response) => {
                                return make_loader_result(
                                    tex_holder_iface,
                                    if response.status() == 400 {
                                        LoaderStatus::ERROR_400
                                    } else if response.status() == 404 {
                                        LoaderStatus::ERROR_404
                                    } else {
                                        println!("error_network");
                                        LoaderStatus::ERROR_NETWORK
                                    },
                                );
                            }
                            _ => return make_loader_result(tex_holder_iface, LoaderStatus::ERROR_OTHER),
                        }
                    }
                }
            };
            let status_code = data.status();

            let Ok(_) = data.into_reader().read_to_end(&mut databytes) else {
                let load_result = TextureHolderInterfaceImpl::default_cpp_owned();
                let tex_holder_iface =
                    TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);
                let tex_holder_iface = transform_texture_holder_interface(tex_holder_iface);
                return make_loader_result(tex_holder_iface, LoaderStatus::ERROR_OTHER);
            };

            std::fs::write(&format!("tiles/{p}"), &databytes);
        } else {
            databytes = std::fs::read(path).unwrap();
        }
        let Ok(image) = image::load_from_memory(&databytes) else {
            let load_result = TextureHolderInterfaceImpl::default_cpp_owned();
            let tex_holder_iface =
                TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);
            let tex_holder_iface = transform_texture_holder_interface(tex_holder_iface);
            return make_loader_result(tex_holder_iface, LoaderStatus::ERROR_OTHER);
        };
        let image_dimensions = image.dimensions();
        let img_buffer = image.to_rgba8();
        let mut interface = TextureHolderInterfaceImpl {
            image_width: image_dimensions.0 as usize,
            image_height: image_dimensions.1 as usize,
            texture_data: img_buffer.to_vec(),
            ..Default::default()
        };
        let mut load_result = TextureHolderInterfaceImpl::new_cpp_owned(interface);
        let tex_holder_iface =
            TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);
        let tex_holder_iface = transform_texture_holder_interface(tex_holder_iface);
        make_loader_result(tex_holder_iface, LoaderStatus::OK)
    }

    fn loadDataWrapper(
        &self,
        url: &cxx::CxxString,
        etag: cxx::UniquePtr<cxx::CxxString>,
    ) -> cxx::UniquePtr<DataLoaderResult> {
        todo!()
    }
}
