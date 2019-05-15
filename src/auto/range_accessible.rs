// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use WidgetAccessible;
use atk;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct RangeAccessible(Object<gtk_sys::GtkRangeAccessible, gtk_sys::GtkRangeAccessibleClass, RangeAccessibleClass>) @extends WidgetAccessible, Accessible, atk::Object;

    match fn {
        get_type => || gtk_sys::gtk_range_accessible_get_type(),
    }
}

impl RangeAccessible {}

pub const NONE_RANGE_ACCESSIBLE: Option<&RangeAccessible> = None;

impl fmt::Display for RangeAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RangeAccessible")
    }
}
