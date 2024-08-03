// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../builddir/rsvg
// from gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, HandleFlags, Rectangle};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "RsvgHandle")]
    pub struct Handle(Object<ffi::RsvgHandle, ffi::RsvgHandleClass>);

    match fn {
        type_ => || ffi::rsvg_handle_get_type(),
    }
}

impl Handle {
    pub const NONE: Option<&'static Handle> = None;

    #[doc(alias = "rsvg_handle_new")]
    pub fn new() -> Handle {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::rsvg_handle_new()) }
    }

    #[doc(alias = "rsvg_handle_new_from_data")]
    #[doc(alias = "new_from_data")]
    pub fn from_data(data: &[u8]) -> Result<Option<Handle>, glib::Error> {
        assert_initialized_main_thread!();
        let data_len = data.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::rsvg_handle_new_from_data(data.to_glib_none().0, data_len, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_new_from_file")]
    #[doc(alias = "new_from_file")]
    pub fn from_file(filename: &str) -> Result<Option<Handle>, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::rsvg_handle_new_from_file(filename.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_new_from_gfile_sync")]
    #[doc(alias = "new_from_gfile_sync")]
    pub fn from_gfile_sync(
        file: &impl IsA<gio::File>,
        flags: HandleFlags,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<Option<Handle>, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::rsvg_handle_new_from_gfile_sync(
                file.as_ref().to_glib_none().0,
                flags.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_new_from_stream_sync")]
    #[doc(alias = "new_from_stream_sync")]
    pub fn from_stream_sync(
        input_stream: &impl IsA<gio::InputStream>,
        base_file: Option<&impl IsA<gio::File>>,
        flags: HandleFlags,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<Option<Handle>, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::rsvg_handle_new_from_stream_sync(
                input_stream.as_ref().to_glib_none().0,
                base_file.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_new_with_flags")]
    #[doc(alias = "new_with_flags")]
    pub fn with_flags(flags: HandleFlags) -> Handle {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::rsvg_handle_new_with_flags(flags.into_glib())) }
    }
}

impl Default for Handle {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Handle>> Sealed for T {}
}

pub trait HandleExt: IsA<Handle> + sealed::Sealed + 'static {
    #[doc(alias = "rsvg_handle_get_base_uri")]
    #[doc(alias = "get_base_uri")]
    #[doc(alias = "base-uri")]
    fn base_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::rsvg_handle_get_base_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "rsvg_handle_get_geometry_for_element")]
    #[doc(alias = "get_geometry_for_element")]
    fn geometry_for_element(
        &self,
        id: Option<&str>,
    ) -> Result<(Rectangle, Rectangle), glib::Error> {
        unsafe {
            let mut out_ink_rect = Rectangle::uninitialized();
            let mut out_logical_rect = Rectangle::uninitialized();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::rsvg_handle_get_geometry_for_element(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                out_ink_rect.to_glib_none_mut().0,
                out_logical_rect.to_glib_none_mut().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((out_ink_rect, out_logical_rect))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_get_geometry_for_layer")]
    #[doc(alias = "get_geometry_for_layer")]
    fn geometry_for_layer(
        &self,
        id: Option<&str>,
        viewport: &Rectangle,
    ) -> Result<(Rectangle, Rectangle), glib::Error> {
        unsafe {
            let mut out_ink_rect = Rectangle::uninitialized();
            let mut out_logical_rect = Rectangle::uninitialized();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::rsvg_handle_get_geometry_for_layer(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                viewport.to_glib_none().0,
                out_ink_rect.to_glib_none_mut().0,
                out_logical_rect.to_glib_none_mut().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((out_ink_rect, out_logical_rect))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_get_intrinsic_size_in_pixels")]
    #[doc(alias = "get_intrinsic_size_in_pixels")]
    fn intrinsic_size_in_pixels(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut out_width = std::mem::MaybeUninit::uninit();
            let mut out_height = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::rsvg_handle_get_intrinsic_size_in_pixels(
                self.as_ref().to_glib_none().0,
                out_width.as_mut_ptr(),
                out_height.as_mut_ptr(),
            ));
            if ret {
                Some((out_width.assume_init(), out_height.assume_init()))
            } else {
                None
            }
        }
    }

    //#[cfg_attr(feature = "v2_58", deprecated = "Since 2.58")]
    //#[allow(deprecated)]
    //#[doc(alias = "rsvg_handle_get_pixbuf")]
    //#[doc(alias = "get_pixbuf")]
    //fn pixbuf(&self) -> /*Ignored*/Option<gdk_pixbuf::Pixbuf> {
    //    unsafe { TODO: call ffi:rsvg_handle_get_pixbuf() }
    //}

    //#[cfg(feature = "v2_58")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v2_58")))]
    //#[doc(alias = "rsvg_handle_get_pixbuf_and_error")]
    //#[doc(alias = "get_pixbuf_and_error")]
    //fn pixbuf_and_error(&self) -> Result</*Ignored*/Option<gdk_pixbuf::Pixbuf>, glib::Error> {
    //    unsafe { TODO: call ffi:rsvg_handle_get_pixbuf_and_error() }
    //}

    //#[doc(alias = "rsvg_handle_get_pixbuf_sub")]
    //#[doc(alias = "get_pixbuf_sub")]
    //fn pixbuf_sub(&self, id: Option<&str>) -> /*Ignored*/Option<gdk_pixbuf::Pixbuf> {
    //    unsafe { TODO: call ffi:rsvg_handle_get_pixbuf_sub() }
    //}

    #[doc(alias = "rsvg_handle_has_sub")]
    fn has_sub(&self, id: &str) -> bool {
        unsafe {
            from_glib(ffi::rsvg_handle_has_sub(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "rsvg_handle_internal_set_testing")]
    fn internal_set_testing(&self, testing: bool) {
        unsafe {
            ffi::rsvg_handle_internal_set_testing(
                self.as_ref().to_glib_none().0,
                testing.into_glib(),
            );
        }
    }

    #[doc(alias = "rsvg_handle_read_stream_sync")]
    fn read_stream_sync(
        &self,
        stream: &impl IsA<gio::InputStream>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::rsvg_handle_read_stream_sync(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_render_document")]
    fn render_document(
        &self,
        cr: &cairo::Context,
        viewport: &Rectangle,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::rsvg_handle_render_document(
                self.as_ref().to_glib_none().0,
                mut_override(cr.to_glib_none().0),
                viewport.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_render_element")]
    fn render_element(
        &self,
        cr: &cairo::Context,
        id: Option<&str>,
        element_viewport: &Rectangle,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::rsvg_handle_render_element(
                self.as_ref().to_glib_none().0,
                mut_override(cr.to_glib_none().0),
                id.to_glib_none().0,
                element_viewport.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_render_layer")]
    fn render_layer(
        &self,
        cr: &cairo::Context,
        id: Option<&str>,
        viewport: &Rectangle,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::rsvg_handle_render_layer(
                self.as_ref().to_glib_none().0,
                mut_override(cr.to_glib_none().0),
                id.to_glib_none().0,
                viewport.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "rsvg_handle_set_base_gfile")]
    fn set_base_gfile(&self, base_file: &impl IsA<gio::File>) {
        unsafe {
            ffi::rsvg_handle_set_base_gfile(
                self.as_ref().to_glib_none().0,
                base_file.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "rsvg_handle_set_base_uri")]
    #[doc(alias = "base-uri")]
    fn set_base_uri(&self, base_uri: &str) {
        unsafe {
            ffi::rsvg_handle_set_base_uri(
                self.as_ref().to_glib_none().0,
                base_uri.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "rsvg_handle_set_cancellable_for_rendering")]
    fn set_cancellable_for_rendering(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) {
        unsafe {
            ffi::rsvg_handle_set_cancellable_for_rendering(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "rsvg_handle_set_dpi")]
    fn set_dpi(&self, dpi: f64) {
        unsafe {
            ffi::rsvg_handle_set_dpi(self.as_ref().to_glib_none().0, dpi);
        }
    }

    #[doc(alias = "rsvg_handle_set_dpi_x_y")]
    fn set_dpi_x_y(&self, dpi_x: f64, dpi_y: f64) {
        unsafe {
            ffi::rsvg_handle_set_dpi_x_y(self.as_ref().to_glib_none().0, dpi_x, dpi_y);
        }
    }

    #[doc(alias = "rsvg_handle_set_stylesheet")]
    fn set_stylesheet(&self, css: &[u8]) -> Result<(), glib::Error> {
        let css_len = css.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::rsvg_handle_set_stylesheet(
                self.as_ref().to_glib_none().0,
                css.to_glib_none().0,
                css_len,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "dpi-x")]
    fn dpi_x(&self) -> f64 {
        ObjectExt::property(self.as_ref(), "dpi-x")
    }

    #[doc(alias = "dpi-x")]
    fn set_dpi_x(&self, dpi_x: f64) {
        ObjectExt::set_property(self.as_ref(), "dpi-x", dpi_x)
    }

    #[doc(alias = "dpi-y")]
    fn dpi_y(&self) -> f64 {
        ObjectExt::property(self.as_ref(), "dpi-y")
    }

    #[doc(alias = "dpi-y")]
    fn set_dpi_y(&self, dpi_y: f64) {
        ObjectExt::set_property(self.as_ref(), "dpi-y", dpi_y)
    }

    fn flags(&self) -> HandleFlags {
        ObjectExt::property(self.as_ref(), "flags")
    }

    #[doc(alias = "base-uri")]
    fn connect_base_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_base_uri_trampoline<P: IsA<Handle>, F: Fn(&P) + 'static>(
            this: *mut ffi::RsvgHandle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Handle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::base-uri\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_base_uri_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "dpi-x")]
    fn connect_dpi_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dpi_x_trampoline<P: IsA<Handle>, F: Fn(&P) + 'static>(
            this: *mut ffi::RsvgHandle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Handle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dpi-x\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_dpi_x_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "dpi-y")]
    fn connect_dpi_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dpi_y_trampoline<P: IsA<Handle>, F: Fn(&P) + 'static>(
            this: *mut ffi::RsvgHandle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Handle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dpi-y\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_dpi_y_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Handle>> HandleExt for O {}
