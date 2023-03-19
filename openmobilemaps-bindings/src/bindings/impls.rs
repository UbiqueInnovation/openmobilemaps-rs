// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::mpsc::Sender;

use autocxx_macro::subclass;
use image::GenericImageView;
use autocxx::subclass::*;

use crate::ffi::*;
use crate::ffi;


#[subclass(superclass("MapReadyCallbackInterface"))]
#[derive(Default)]
pub struct MapReadyCallbackInterfaceImpl {
    pub sender: Option<Sender<LayerReadyState>>,
}

impl MapReadyCallbackInterface_methods for MapReadyCallbackInterfaceImpl {
    fn stateDidUpdate(&mut self, state: LayerReadyState) {
        match state {
            LayerReadyState::READY => println!("READY"),
            LayerReadyState::NOT_READY => {}
            LayerReadyState::ERROR => println!("ERROR"),
            LayerReadyState::TIMEOUT_ERROR => println!("TIMEOUT_ERROR"),
        }
        if let Some(sender) = self.sender.as_ref() {
            let _ = sender.send(state);
        }
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
    image_data: Vec<u8>,
    texture_data: Vec<u8>,
    usage_counter: usize,
    id: u32,
    attached: bool,
    texture: Option<glium::texture::SrgbTexture2d>,
    display: Option<glium::Display>,
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
            unsafe { gl::DeleteTextures(1, &mut self.id) };
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
    pub display: Option<glium::Display>,
}

impl LoaderInterfaceImpl_methods for LoaderInterfaceImplRs {
    fn loadTextureWrapper(
        &self,
        url: &cxx::CxxString,
        etag: cxx::UniquePtr<cxx::CxxString>,
    ) -> cxx::UniquePtr<TextureLoaderResult> {
        // println!("In load texture interface");
        let Ok(data) = ureq::get(url.to_str().unwrap()).call() else {
            let load_result = TextureHolderInterfaceImpl::default_cpp_owned();
            let tex_holder_iface =
            TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);
            let tex_holder_iface = transform_texture_holder_interface(tex_holder_iface);
            return make_loader_result(tex_holder_iface, LoaderStatus::ERROR_OTHER);
        };
        let mut databytes = vec![];
        let Ok(_)=  data.into_reader().read_to_end(&mut databytes) else {
            let load_result = TextureHolderInterfaceImpl::default_cpp_owned();
            let tex_holder_iface =
            TextureHolderInterfaceImpl::as_TextureHolderInterface_unique_ptr(load_result);
            let tex_holder_iface = transform_texture_holder_interface(tex_holder_iface);
            return make_loader_result(tex_holder_iface, LoaderStatus::ERROR_OTHER);
        };
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
            image_data: databytes,
            texture_data: img_buffer.to_vec(),
            display: Some(self.display.as_ref().unwrap().clone()),

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
