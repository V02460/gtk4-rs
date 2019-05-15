// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use ButtonAccessible;
use ContainerAccessible;
use WidgetAccessible;
use atk;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct ToggleButtonAccessible(Object<gtk_sys::GtkToggleButtonAccessible, gtk_sys::GtkToggleButtonAccessibleClass, ToggleButtonAccessibleClass>) @extends ButtonAccessible, ContainerAccessible, WidgetAccessible, Accessible, atk::Object;

    match fn {
        get_type => || gtk_sys::gtk_toggle_button_accessible_get_type(),
    }
}

impl ToggleButtonAccessible {}

pub const NONE_TOGGLE_BUTTON_ACCESSIBLE: Option<&ToggleButtonAccessible> = None;

impl fmt::Display for ToggleButtonAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ToggleButtonAccessible")
    }
}
