mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

glib::wrapper! {
    pub struct TodoObject(ObjectSubclass<imp::TodoObject>);
}

impl TodoObject {
    pub fn new(content: String, completed: bool) -> Self {
        Object::new(&[("content", &content), ("completed", &completed)])
            .expect("Failed to create `TodoObject`.")
    }

    pub fn todo_data(&self) -> Rc<RefCell<TodoData>> {
        let imp = imp::TodoObject::from_instance(self);
        imp.data.clone()
    }
}

#[derive(Default)]
pub struct TodoData {
    pub completed: bool,
    pub content: String,
}
