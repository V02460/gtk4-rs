// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use ArrowType;
use Bin;
use Buildable;
use Button;
use Container;
use Menu;
use Popover;
use ToggleButton;
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
    pub struct MenuButton(Object<gtk_sys::GtkMenuButton, gtk_sys::GtkMenuButtonClass, MenuButtonClass>) @extends ToggleButton, Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || gtk_sys::gtk_menu_button_get_type(),
    }
}

impl MenuButton {
    pub fn new() -> MenuButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_menu_button_new()).unsafe_cast()
        }
    }
}

impl Default for MenuButton {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MENU_BUTTON: Option<&MenuButton> = None;

pub trait MenuButtonExt: 'static {
    fn get_align_widget(&self) -> Option<Widget>;

    fn get_direction(&self) -> ArrowType;

    //fn get_menu_model(&self) -> /*Ignored*/Option<gio::MenuModel>;

    fn get_popover(&self) -> Option<Popover>;

    fn get_popup(&self) -> Option<Menu>;

    fn get_use_popover(&self) -> bool;

    fn set_align_widget<P: IsA<Widget>>(&self, align_widget: Option<&P>);

    fn set_direction(&self, direction: ArrowType);

    //fn set_menu_model(&self, menu_model: /*Ignored*/Option<&gio::MenuModel>);

    fn set_popover<P: IsA<Widget>>(&self, popover: Option<&P>);

    fn set_popup<P: IsA<Widget>>(&self, menu: Option<&P>);

    fn set_use_popover(&self, use_popover: bool);

    fn connect_property_align_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuButton>> MenuButtonExt for O {
    fn get_align_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_button_get_align_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_direction(&self) -> ArrowType {
        unsafe {
            from_glib(gtk_sys::gtk_menu_button_get_direction(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_menu_model(&self) -> /*Ignored*/Option<gio::MenuModel> {
    //    unsafe { TODO: call gtk_sys:gtk_menu_button_get_menu_model() }
    //}

    fn get_popover(&self) -> Option<Popover> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_button_get_popover(self.as_ref().to_glib_none().0))
        }
    }

    fn get_popup(&self) -> Option<Menu> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_button_get_popup(self.as_ref().to_glib_none().0))
        }
    }

    fn get_use_popover(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_menu_button_get_use_popover(self.as_ref().to_glib_none().0))
        }
    }

    fn set_align_widget<P: IsA<Widget>>(&self, align_widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_button_set_align_widget(self.as_ref().to_glib_none().0, align_widget.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_direction(&self, direction: ArrowType) {
        unsafe {
            gtk_sys::gtk_menu_button_set_direction(self.as_ref().to_glib_none().0, direction.to_glib());
        }
    }

    //fn set_menu_model(&self, menu_model: /*Ignored*/Option<&gio::MenuModel>) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_button_set_menu_model() }
    //}

    fn set_popover<P: IsA<Widget>>(&self, popover: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_button_set_popover(self.as_ref().to_glib_none().0, popover.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_popup<P: IsA<Widget>>(&self, menu: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_button_set_popup(self.as_ref().to_glib_none().0, menu.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_use_popover(&self, use_popover: bool) {
        unsafe {
            gtk_sys::gtk_menu_button_set_use_popover(self.as_ref().to_glib_none().0, use_popover.to_glib());
        }
    }

    fn connect_property_align_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::align-widget\0".as_ptr() as *const _,
                Some(transmute(notify_align_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::direction\0".as_ptr() as *const _,
                Some(transmute(notify_direction_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::menu-model\0".as_ptr() as *const _,
                Some(transmute(notify_menu_model_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::popover\0".as_ptr() as *const _,
                Some(transmute(notify_popover_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::popup\0".as_ptr() as *const _,
                Some(transmute(notify_popup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-popover\0".as_ptr() as *const _,
                Some(transmute(notify_use_popover_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_align_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenuButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<MenuButton> {
    let f: &F = &*(f as *const F);
    f(&MenuButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_direction_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenuButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<MenuButton> {
    let f: &F = &*(f as *const F);
    f(&MenuButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_menu_model_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenuButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<MenuButton> {
    let f: &F = &*(f as *const F);
    f(&MenuButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_popover_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenuButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<MenuButton> {
    let f: &F = &*(f as *const F);
    f(&MenuButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_popup_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenuButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<MenuButton> {
    let f: &F = &*(f as *const F);
    f(&MenuButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_popover_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenuButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<MenuButton> {
    let f: &F = &*(f as *const F);
    f(&MenuButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for MenuButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuButton")
    }
}
