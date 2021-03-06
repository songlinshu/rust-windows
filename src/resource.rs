// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use std;

use user32;
use winapi::{HANDLE, UINT, c_int};

use wchar::ToCU16Str;
use instance::Instance;

pub trait ToHandle {
    fn to_handle(&self) -> HANDLE;
}

impl<T: ToHandle> ToHandle for Option<T> {
    fn to_handle(&self) -> HANDLE {
        match *self {
            None => ptr::null_mut(),
            Some(ref s) => s.to_handle(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub enum ImageType {
    IMAGE_BITMAP = 0,
    IMAGE_ICON = 1,
    IMAGE_CURSOR = 2,
}

#[derive(Clone,Copy)]
pub struct Image {
    pub image: HANDLE,
}

impl Image {
    pub fn load_resource(instance: Instance, id: isize, img_type: ImageType, width: isize, height: isize) -> Option<Image> {
        let img = unsafe {
            user32::LoadImageW(
                instance.instance, std::mem::transmute(id), img_type as UINT,
                width as c_int, height as c_int, 0x8000
            )
        };

        if img == ptr::null_mut() {
            None
        } else {
            Some(Image { image: img })
        }
    }

    pub fn load_cursor_resource(id: isize) -> Option<Image> {
        let null_instance = Instance { instance: ptr::null_mut() };
        Image::load_resource(null_instance, id, ImageType::IMAGE_CURSOR, 0, 0)
    }
}

impl ToHandle for Image {
    fn to_handle(&self) -> HANDLE {
        self.image
    }
}

pub enum MenuResource {
    MenuName(String),
    MenuId(isize),
}

impl MenuResource {
    pub fn with_menu_p<T, F>(&self, f: F) -> T
        where F: FnOnce(*const u16) -> T {
        match *self {
            MenuResource::MenuName(ref s) => {
                let u = &s.to_c_u16();
                f(u.as_ptr())
            }
            MenuResource::MenuId(id) => unsafe { f(std::mem::transmute(id)) },
        }
    }

    pub fn null() -> MenuResource {
        MenuResource::MenuId(0)
    }
}
