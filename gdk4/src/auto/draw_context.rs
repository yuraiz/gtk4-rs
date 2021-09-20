// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Display;
use crate::Surface;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkDrawContext")]
    pub struct DrawContext(Object<ffi::GdkDrawContext>);

    match fn {
        type_ => || ffi::gdk_draw_context_get_type(),
    }
}

pub const NONE_DRAW_CONTEXT: Option<&DrawContext> = None;

pub trait DrawContextExt: 'static {
    #[doc(alias = "gdk_draw_context_begin_frame")]
    fn begin_frame(&self, region: &cairo::Region);

    #[doc(alias = "gdk_draw_context_end_frame")]
    fn end_frame(&self);

    #[doc(alias = "gdk_draw_context_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display>;

    #[doc(alias = "gdk_draw_context_get_surface")]
    #[doc(alias = "get_surface")]
    fn surface(&self) -> Option<Surface>;

    #[doc(alias = "gdk_draw_context_is_in_frame")]
    fn is_in_frame(&self) -> bool;
}

impl<O: IsA<DrawContext>> DrawContextExt for O {
    fn begin_frame(&self, region: &cairo::Region) {
        unsafe {
            ffi::gdk_draw_context_begin_frame(
                self.as_ref().to_glib_none().0,
                region.to_glib_none().0,
            );
        }
    }

    fn end_frame(&self) {
        unsafe {
            ffi::gdk_draw_context_end_frame(self.as_ref().to_glib_none().0);
        }
    }

    fn display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_draw_context_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn surface(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(ffi::gdk_draw_context_get_surface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_in_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_draw_context_is_in_frame(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for DrawContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DrawContext")
    }
}
