use std::ffi::OsStr;

use libloading::Library;
use ouroboros::self_referencing;

fn main() {
    let my_plugin = MyPlugin::load("target/debug/libmylib.so").unwrap();

    my_plugin.hello();

    let val = my_plugin.ret_val();

    println!("val: {val:?}")
}

#[self_referencing]
pub struct MyPlugin {
    lib: libloading::Library,
    #[borrows(lib)]
    #[covariant]
    hello: libloading::Symbol<'this, unsafe extern "C" fn()>,

    #[borrows(lib)]
    #[covariant]
    ret_val: libloading::Symbol<'this, Option<unsafe extern "C" fn() -> u32>>,
}

impl MyPlugin {
    pub fn load<P: AsRef<OsStr>>(filename: P) -> Result<Self, libloading::Error> {
        unsafe {
            let lib = libloading::Library::new(filename)?;

            MyPluginTryBuilder {
                lib,
                hello_builder: |lib: &Library| lib.get(b"hello"),
                ret_val_builder: |lib: &Library| lib.get(b"return_value"),
            }
            .try_build()
        }
    }

    /// Просто печатает 'Hello'
    pub fn hello(&self) {
        unsafe { self.borrow_hello()() }
    }
    /// Возвращает u32, если удалось найти такую функцию в библиотеке
    pub fn ret_val(&self) -> Option<u32> {
        unsafe { self.borrow_ret_val().map(|f| f()) }
    }
}
