// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Orientable;
use Orientation;
use PositionType;
use Range;
use Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Scale(Object<gtk_sys::GtkScale, gtk_sys::GtkScaleClass, ScaleClass>) @extends Range, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_scale_get_type(),
    }
}

impl Scale {
    pub fn new<P: IsA<Adjustment>>(orientation: Orientation, adjustment: Option<&P>) -> Scale {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_scale_new(orientation.to_glib(), adjustment.map(|p| p.as_ref()).to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_range(orientation: Orientation, min: f64, max: f64, step: f64) -> Scale {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_scale_new_with_range(orientation.to_glib(), min, max, step)).unsafe_cast()
        }
    }
}

pub const NONE_SCALE: Option<&Scale> = None;

pub trait ScaleExt: 'static {
    fn add_mark(&self, value: f64, position: PositionType, markup: Option<&str>);

    fn clear_marks(&self);

    fn get_digits(&self) -> i32;

    fn get_draw_value(&self) -> bool;

    fn get_has_origin(&self) -> bool;

    //fn get_layout(&self) -> /*Ignored*/Option<pango::Layout>;

    fn get_layout_offsets(&self) -> (i32, i32);

    fn get_value_pos(&self) -> PositionType;

    fn set_digits(&self, digits: i32);

    fn set_draw_value(&self, draw_value: bool);

    fn set_has_origin(&self, has_origin: bool);

    fn set_value_pos(&self, pos: PositionType);

    fn connect_format_value<F: Fn(&Self, f64) -> String + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_draw_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Scale>> ScaleExt for O {
    fn add_mark(&self, value: f64, position: PositionType, markup: Option<&str>) {
        unsafe {
            gtk_sys::gtk_scale_add_mark(self.as_ref().to_glib_none().0, value, position.to_glib(), markup.to_glib_none().0);
        }
    }

    fn clear_marks(&self) {
        unsafe {
            gtk_sys::gtk_scale_clear_marks(self.as_ref().to_glib_none().0);
        }
    }

    fn get_digits(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_scale_get_digits(self.as_ref().to_glib_none().0)
        }
    }

    fn get_draw_value(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_scale_get_draw_value(self.as_ref().to_glib_none().0))
        }
    }

    fn get_has_origin(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_scale_get_has_origin(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_layout(&self) -> /*Ignored*/Option<pango::Layout> {
    //    unsafe { TODO: call gtk_sys:gtk_scale_get_layout() }
    //}

    fn get_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            gtk_sys::gtk_scale_get_layout_offsets(self.as_ref().to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    fn get_value_pos(&self) -> PositionType {
        unsafe {
            from_glib(gtk_sys::gtk_scale_get_value_pos(self.as_ref().to_glib_none().0))
        }
    }

    fn set_digits(&self, digits: i32) {
        unsafe {
            gtk_sys::gtk_scale_set_digits(self.as_ref().to_glib_none().0, digits);
        }
    }

    fn set_draw_value(&self, draw_value: bool) {
        unsafe {
            gtk_sys::gtk_scale_set_draw_value(self.as_ref().to_glib_none().0, draw_value.to_glib());
        }
    }

    fn set_has_origin(&self, has_origin: bool) {
        unsafe {
            gtk_sys::gtk_scale_set_has_origin(self.as_ref().to_glib_none().0, has_origin.to_glib());
        }
    }

    fn set_value_pos(&self, pos: PositionType) {
        unsafe {
            gtk_sys::gtk_scale_set_value_pos(self.as_ref().to_glib_none().0, pos.to_glib());
        }
    }

    fn connect_format_value<F: Fn(&Self, f64) -> String + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"format-value\0".as_ptr() as *const _,
                Some(transmute(format_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::digits\0".as_ptr() as *const _,
                Some(transmute(notify_digits_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_draw_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::draw-value\0".as_ptr() as *const _,
                Some(transmute(notify_draw_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_has_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::has-origin\0".as_ptr() as *const _,
                Some(transmute(notify_has_origin_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value-pos\0".as_ptr() as *const _,
                Some(transmute(notify_value_pos_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn format_value_trampoline<P, F: Fn(&P, f64) -> String + 'static>(this: *mut gtk_sys::GtkScale, value: libc::c_double, f: glib_sys::gpointer) -> *mut libc::c_char
where P: IsA<Scale> {
    let f: &F = &*(f as *const F);
    f(&Scale::from_glib_borrow(this).unsafe_cast(), value).to_glib_full()
}

unsafe extern "C" fn notify_digits_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScale, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Scale> {
    let f: &F = &*(f as *const F);
    f(&Scale::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_draw_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScale, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Scale> {
    let f: &F = &*(f as *const F);
    f(&Scale::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_has_origin_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScale, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Scale> {
    let f: &F = &*(f as *const F);
    f(&Scale::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_pos_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScale, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Scale> {
    let f: &F = &*(f as *const F);
    f(&Scale::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scale")
    }
}
