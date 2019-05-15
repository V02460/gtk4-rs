// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use PopoverConstraint;
use PositionType;
use Widget;
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
    pub struct Popover(Object<gtk_sys::GtkPopover, gtk_sys::GtkPopoverClass, PopoverClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_popover_get_type(),
    }
}

impl Popover {
    pub fn new<P: IsA<Widget>>(relative_to: Option<&P>) -> Popover {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_popover_new(relative_to.map(|p| p.as_ref()).to_glib_none().0)).unsafe_cast()
        }
    }

    //pub fn new_from_model<P: IsA<Widget>>(relative_to: Option<&P>, model: /*Ignored*/&gio::MenuModel) -> Popover {
    //    unsafe { TODO: call gtk_sys:gtk_popover_new_from_model() }
    //}
}

pub const NONE_POPOVER: Option<&Popover> = None;

pub trait PopoverExt: 'static {
    //fn bind_model(&self, model: /*Ignored*/Option<&gio::MenuModel>, action_namespace: Option<&str>);

    fn get_constrain_to(&self) -> PopoverConstraint;

    fn get_default_widget(&self) -> Option<Widget>;

    fn get_modal(&self) -> bool;

    //fn get_pointing_to(&self, rect: /*Ignored*/gdk::Rectangle) -> bool;

    fn get_position(&self) -> PositionType;

    fn get_relative_to(&self) -> Option<Widget>;

    fn popdown(&self);

    fn popup(&self);

    fn set_constrain_to(&self, constraint: PopoverConstraint);

    fn set_default_widget<P: IsA<Widget>>(&self, widget: Option<&P>);

    fn set_modal(&self, modal: bool);

    //fn set_pointing_to(&self, rect: /*Ignored*/&gdk::Rectangle);

    fn set_position(&self, position: PositionType);

    fn set_relative_to<P: IsA<Widget>>(&self, relative_to: Option<&P>);

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_constrain_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_relative_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Popover>> PopoverExt for O {
    //fn bind_model(&self, model: /*Ignored*/Option<&gio::MenuModel>, action_namespace: Option<&str>) {
    //    unsafe { TODO: call gtk_sys:gtk_popover_bind_model() }
    //}

    fn get_constrain_to(&self) -> PopoverConstraint {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_constrain_to(self.as_ref().to_glib_none().0))
        }
    }

    fn get_default_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_popover_get_default_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_modal(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_pointing_to(&self, rect: /*Ignored*/gdk::Rectangle) -> bool {
    //    unsafe { TODO: call gtk_sys:gtk_popover_get_pointing_to() }
    //}

    fn get_position(&self) -> PositionType {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_position(self.as_ref().to_glib_none().0))
        }
    }

    fn get_relative_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_popover_get_relative_to(self.as_ref().to_glib_none().0))
        }
    }

    fn popdown(&self) {
        unsafe {
            gtk_sys::gtk_popover_popdown(self.as_ref().to_glib_none().0);
        }
    }

    fn popup(&self) {
        unsafe {
            gtk_sys::gtk_popover_popup(self.as_ref().to_glib_none().0);
        }
    }

    fn set_constrain_to(&self, constraint: PopoverConstraint) {
        unsafe {
            gtk_sys::gtk_popover_set_constrain_to(self.as_ref().to_glib_none().0, constraint.to_glib());
        }
    }

    fn set_default_widget<P: IsA<Widget>>(&self, widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_popover_set_default_widget(self.as_ref().to_glib_none().0, widget.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            gtk_sys::gtk_popover_set_modal(self.as_ref().to_glib_none().0, modal.to_glib());
        }
    }

    //fn set_pointing_to(&self, rect: /*Ignored*/&gdk::Rectangle) {
    //    unsafe { TODO: call gtk_sys:gtk_popover_set_pointing_to() }
    //}

    fn set_position(&self, position: PositionType) {
        unsafe {
            gtk_sys::gtk_popover_set_position(self.as_ref().to_glib_none().0, position.to_glib());
        }
    }

    fn set_relative_to<P: IsA<Widget>>(&self, relative_to: Option<&P>) {
        unsafe {
            gtk_sys::gtk_popover_set_relative_to(self.as_ref().to_glib_none().0, relative_to.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"closed\0".as_ptr() as *const _,
                Some(transmute(closed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_constrain_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::constrain-to\0".as_ptr() as *const _,
                Some(transmute(notify_constrain_to_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_default_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::default-widget\0".as_ptr() as *const _,
                Some(transmute(notify_default_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::modal\0".as_ptr() as *const _,
                Some(transmute(notify_modal_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pointing-to\0".as_ptr() as *const _,
                Some(transmute(notify_pointing_to_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::position\0".as_ptr() as *const _,
                Some(transmute(notify_position_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_relative_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::relative-to\0".as_ptr() as *const _,
                Some(transmute(notify_relative_to_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn closed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPopover, f: glib_sys::gpointer)
where P: IsA<Popover> {
    let f: &F = &*(f as *const F);
    f(&Popover::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_constrain_to_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPopover, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Popover> {
    let f: &F = &*(f as *const F);
    f(&Popover::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_default_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPopover, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Popover> {
    let f: &F = &*(f as *const F);
    f(&Popover::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_modal_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPopover, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Popover> {
    let f: &F = &*(f as *const F);
    f(&Popover::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pointing_to_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPopover, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Popover> {
    let f: &F = &*(f as *const F);
    f(&Popover::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_position_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPopover, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Popover> {
    let f: &F = &*(f as *const F);
    f(&Popover::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_relative_to_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPopover, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Popover> {
    let f: &F = &*(f as *const F);
    f(&Popover::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Popover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Popover")
    }
}
