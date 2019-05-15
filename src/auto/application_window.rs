// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Application;
use Bin;
use Buildable;
use Container;
use Root;
use ShortcutsWindow;
use Widget;
use Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ApplicationWindow(Object<gtk_sys::GtkApplicationWindow, gtk_sys::GtkApplicationWindowClass, ApplicationWindowClass>) @extends Window, Bin, Container, Widget, @implements Buildable, Root;

    match fn {
        get_type => || gtk_sys::gtk_application_window_get_type(),
    }
}

impl ApplicationWindow {
    pub fn new<P: IsA<Application>>(application: &P) -> ApplicationWindow {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_application_window_new(application.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_APPLICATION_WINDOW: Option<&ApplicationWindow> = None;

pub trait ApplicationWindowExt: 'static {
    fn get_help_overlay(&self) -> Option<ShortcutsWindow>;

    fn get_id(&self) -> u32;

    fn get_show_menubar(&self) -> bool;

    fn set_help_overlay<P: IsA<ShortcutsWindow>>(&self, help_overlay: Option<&P>);

    fn set_show_menubar(&self, show_menubar: bool);

    fn connect_property_show_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ApplicationWindow>> ApplicationWindowExt for O {
    fn get_help_overlay(&self) -> Option<ShortcutsWindow> {
        unsafe {
            from_glib_none(gtk_sys::gtk_application_window_get_help_overlay(self.as_ref().to_glib_none().0))
        }
    }

    fn get_id(&self) -> u32 {
        unsafe {
            gtk_sys::gtk_application_window_get_id(self.as_ref().to_glib_none().0)
        }
    }

    fn get_show_menubar(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_application_window_get_show_menubar(self.as_ref().to_glib_none().0))
        }
    }

    fn set_help_overlay<P: IsA<ShortcutsWindow>>(&self, help_overlay: Option<&P>) {
        unsafe {
            gtk_sys::gtk_application_window_set_help_overlay(self.as_ref().to_glib_none().0, help_overlay.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_show_menubar(&self, show_menubar: bool) {
        unsafe {
            gtk_sys::gtk_application_window_set_show_menubar(self.as_ref().to_glib_none().0, show_menubar.to_glib());
        }
    }

    fn connect_property_show_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-menubar\0".as_ptr() as *const _,
                Some(transmute(notify_show_menubar_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_show_menubar_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkApplicationWindow, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ApplicationWindow> {
    let f: &F = &*(f as *const F);
    f(&ApplicationWindow::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ApplicationWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApplicationWindow")
    }
}
