// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use ReliefStyle;
use Widget;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Button(Object<gtk_sys::GtkButton, gtk_sys::GtkButtonClass, ButtonClass>) @extends Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || gtk_sys::gtk_button_get_type(),
    }
}

impl Button {
    pub fn new() -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_button_new()).unsafe_cast()
        }
    }

    pub fn new_from_icon_name(icon_name: Option<&str>) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_button_new_from_icon_name(icon_name.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_label(label: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_button_new_with_label(label.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_button_new_with_mnemonic(label.to_glib_none().0)).unsafe_cast()
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_BUTTON: Option<&Button> = None;

pub trait ButtonExt: 'static {
    fn clicked(&self);

    fn get_icon_name(&self) -> Option<GString>;

    fn get_label(&self) -> Option<GString>;

    fn get_relief(&self) -> ReliefStyle;

    fn get_use_underline(&self) -> bool;

    fn set_icon_name(&self, icon_name: &str);

    fn set_label(&self, label: &str);

    fn set_relief(&self, relief: ReliefStyle);

    fn set_use_underline(&self, use_underline: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_clicked(&self);

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Button>> ButtonExt for O {
    fn clicked(&self) {
        unsafe {
            gtk_sys::gtk_button_clicked(self.as_ref().to_glib_none().0);
        }
    }

    fn get_icon_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_button_get_icon_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_button_get_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_relief(&self) -> ReliefStyle {
        unsafe {
            from_glib(gtk_sys::gtk_button_get_relief(self.as_ref().to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_button_get_use_underline(self.as_ref().to_glib_none().0))
        }
    }

    fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            gtk_sys::gtk_button_set_icon_name(self.as_ref().to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            gtk_sys::gtk_button_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_relief(&self, relief: ReliefStyle) {
        unsafe {
            gtk_sys::gtk_button_set_relief(self.as_ref().to_glib_none().0, relief.to_glib());
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            gtk_sys::gtk_button_set_use_underline(self.as_ref().to_glib_none().0, use_underline.to_glib());
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate\0".as_ptr() as *const _,
                Some(transmute(activate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_activate(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("activate", &[]).unwrap() };
    }

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"clicked\0".as_ptr() as *const _,
                Some(transmute(clicked_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_clicked(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("clicked", &[]).unwrap() };
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute(notify_icon_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::relief\0".as_ptr() as *const _,
                Some(transmute(notify_relief_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute(notify_use_underline_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkButton, f: glib_sys::gpointer)
where P: IsA<Button> {
    let f: &F = &*(f as *const F);
    f(&Button::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn clicked_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkButton, f: glib_sys::gpointer)
where P: IsA<Button> {
    let f: &F = &*(f as *const F);
    f(&Button::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Button> {
    let f: &F = &*(f as *const F);
    f(&Button::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Button> {
    let f: &F = &*(f as *const F);
    f(&Button::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_relief_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Button> {
    let f: &F = &*(f as *const F);
    f(&Button::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_underline_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Button> {
    let f: &F = &*(f as *const F);
    f(&Button::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Button")
    }
}
