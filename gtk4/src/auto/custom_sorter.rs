// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use Sorter;

glib_wrapper! {
    pub struct CustomSorter(Object<gtk_sys::GtkCustomSorter, gtk_sys::GtkCustomSorterClass>) @extends Sorter;

    match fn {
        get_type => || gtk_sys::gtk_custom_sorter_get_type(),
    }
}

impl CustomSorter {
    //pub fn new(sort_func: /*Unimplemented*/Fn(/*Unimplemented*/Option<Fundamental: Pointer>, /*Unimplemented*/Option<Fundamental: Pointer>) -> i32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> CustomSorter {
    //    unsafe { TODO: call gtk_sys:gtk_custom_sorter_new() }
    //}
}

pub const NONE_CUSTOM_SORTER: Option<&CustomSorter> = None;

pub trait CustomSorterExt: 'static {
    //fn set_sort_func(&self, sort_func: /*Unimplemented*/Fn(/*Unimplemented*/Option<Fundamental: Pointer>, /*Unimplemented*/Option<Fundamental: Pointer>) -> i32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);
}

impl<O: IsA<CustomSorter>> CustomSorterExt for O {
    //fn set_sort_func(&self, sort_func: /*Unimplemented*/Fn(/*Unimplemented*/Option<Fundamental: Pointer>, /*Unimplemented*/Option<Fundamental: Pointer>) -> i32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gtk_sys:gtk_custom_sorter_set_sort_func() }
    //}
}

impl fmt::Display for CustomSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomSorter")
    }
}