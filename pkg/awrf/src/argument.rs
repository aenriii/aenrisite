use std::{alloc::Layout, ops::Deref};


// FIXME: literally ANY other way to do this
pub struct Argument {
    type_id: Option<String>,
    container: Option<Box<*mut u8>>,
    value_data: Option<(usize, usize)>
}

impl Argument {
    pub fn new() -> Self {
        Self {
            type_id: None,
            container: None,
            value_data: None
        }
    }
    /// Inserts a variable into the argument, returning
    /// what it was provided if it's already been filled
    pub fn insert<T>(&mut self, it: T) -> Option<T> {

        if let Some(_a) = &self.container {
            return Some(it);
        }

        self.value_data = Some((
            std::mem::size_of_val(&it),
            std::mem::align_of_val(&it)
        ));
        let type_id = std::any::type_name::<T>();

        // edge case: &str literals
        if type_id == std::any::type_name_of_val(&"") {
            // unsafe cast to &str
            unsafe {
                let it = *(&it as *const T as *const u8 as *const &str);
                self.insert(it.to_string());
                return None
            }
        }

        let container = Box::leak(Box::new(it)) as *mut T;
        self.type_id = Some(type_id.to_string());
        self.container = Some(Box::new(container as *mut u8));

        None
    }

    /// Tries to get a variable of type T from this argument,
    /// consumes the container if the type matches
    pub fn try_get<T>(&mut self) -> Option<T> {

        if let None = &self.container {
            return None
        }

        let type_id = std::any::type_name::<T>();

        if let Some(inserted_type) = &self.type_id && inserted_type == &(type_id.to_string()) {
            if let Some(the_box) = &self.container { unsafe {
                let handle_handle = the_box.deref() as *const *mut u8;
                let handle = *handle_handle;
                let item = (handle as *mut T).read();
                self.container = None;
                self.value_data = None;
                return Some(item);
            } }
        }
        None
    }
}

impl Drop for Argument {
    fn drop(&mut self) {
        if let Some(the_box) = &self.container &&
           let Some((size, align)) = self.value_data &&
           let Ok(layout) = Layout::from_size_align(size, align)
        { unsafe {
            let handle_handle = the_box.deref() as *const *mut u8;
            let handle = *handle_handle;

            std::alloc::dealloc(handle, layout);
        } }
    }
}
