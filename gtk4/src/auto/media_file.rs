// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::MediaStream;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkMediaFile")]
    pub struct MediaFile(Object<ffi::GtkMediaFile, ffi::GtkMediaFileClass>) @extends MediaStream, @implements gdk::Paintable;

    match fn {
        type_ => || ffi::gtk_media_file_get_type(),
    }
}

impl MediaFile {
    #[doc(alias = "gtk_media_file_new")]
    pub fn new() -> MediaFile {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_media_file_new()) }
    }

    #[doc(alias = "gtk_media_file_new_for_file")]
    #[doc(alias = "new_for_file")]
    pub fn for_file<P: IsA<gio::File>>(file: &P) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_media_file_new_for_file(
                file.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_media_file_new_for_filename")]
    #[doc(alias = "new_for_filename")]
    pub fn for_filename<P: AsRef<std::path::Path>>(filename: P) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_media_file_new_for_filename(
                filename.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_media_file_new_for_input_stream")]
    #[doc(alias = "new_for_input_stream")]
    pub fn for_input_stream<P: IsA<gio::InputStream>>(stream: &P) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_media_file_new_for_input_stream(
                stream.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_media_file_new_for_resource")]
    #[doc(alias = "new_for_resource")]
    pub fn for_resource(resource_path: &str) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_media_file_new_for_resource(
                resource_path.to_glib_none().0,
            ))
        }
    }
}

impl Default for MediaFile {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MEDIA_FILE: Option<&MediaFile> = None;

pub trait MediaFileExt: 'static {
    #[doc(alias = "gtk_media_file_clear")]
    fn clear(&self);

    #[doc(alias = "gtk_media_file_get_file")]
    #[doc(alias = "get_file")]
    fn file(&self) -> Option<gio::File>;

    #[doc(alias = "gtk_media_file_get_input_stream")]
    #[doc(alias = "get_input_stream")]
    fn input_stream(&self) -> Option<gio::InputStream>;

    #[doc(alias = "gtk_media_file_set_file")]
    fn set_file<P: IsA<gio::File>>(&self, file: Option<&P>);

    #[doc(alias = "gtk_media_file_set_filename")]
    fn set_filename<P: AsRef<std::path::Path>>(&self, filename: P);

    #[doc(alias = "gtk_media_file_set_input_stream")]
    fn set_input_stream<P: IsA<gio::InputStream>>(&self, stream: Option<&P>);

    #[doc(alias = "gtk_media_file_set_resource")]
    fn set_resource(&self, resource_path: Option<&str>);

    #[doc(alias = "file")]
    fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "input-stream")]
    fn connect_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MediaFile>> MediaFileExt for O {
    fn clear(&self) {
        unsafe {
            ffi::gtk_media_file_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn file(&self) -> Option<gio::File> {
        unsafe { from_glib_none(ffi::gtk_media_file_get_file(self.as_ref().to_glib_none().0)) }
    }

    fn input_stream(&self) -> Option<gio::InputStream> {
        unsafe {
            from_glib_none(ffi::gtk_media_file_get_input_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_file<P: IsA<gio::File>>(&self, file: Option<&P>) {
        unsafe {
            ffi::gtk_media_file_set_file(
                self.as_ref().to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_filename<P: AsRef<std::path::Path>>(&self, filename: P) {
        unsafe {
            ffi::gtk_media_file_set_filename(
                self.as_ref().to_glib_none().0,
                filename.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_input_stream<P: IsA<gio::InputStream>>(&self, stream: Option<&P>) {
        unsafe {
            ffi::gtk_media_file_set_input_stream(
                self.as_ref().to_glib_none().0,
                stream.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_media_file_set_resource(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
            );
        }
    }

    fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<P: IsA<MediaFile>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaFile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaFile::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::file\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_stream_trampoline<
            P: IsA<MediaFile>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMediaFile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaFile::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-stream\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_stream_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MediaFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MediaFile")
    }
}
