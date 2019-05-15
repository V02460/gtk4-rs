// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use ContainerAccessible;
use WidgetAccessible;
use atk;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct FlowBoxAccessible(Object<gtk_sys::GtkFlowBoxAccessible, gtk_sys::GtkFlowBoxAccessibleClass, FlowBoxAccessibleClass>) @extends ContainerAccessible, WidgetAccessible, Accessible, atk::Object;

    match fn {
        get_type => || gtk_sys::gtk_flow_box_accessible_get_type(),
    }
}

impl FlowBoxAccessible {}

pub const NONE_FLOW_BOX_ACCESSIBLE: Option<&FlowBoxAccessible> = None;

impl fmt::Display for FlowBoxAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FlowBoxAccessible")
    }
}
