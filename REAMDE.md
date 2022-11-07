# Пример красивой реализации динамически подключаемого плагина

## Какие крейты я использовал:

- [libloading](https://crates.io/crates/libloading) для динамической загрузки
  плагина
- [ouroboros](https://crates.io/crates/ouroboros) для самореферных структур.
  Позволяет реализовать чистый тип плагина

## Структура подключаемой библиотеки

Сама библиотека содержит две функции

```rust
#[no_mangle]
pub fn hello() {
    println!("Hello");
}

#[no_mangle]
pub fn return_value() -> u32 {
    155
}
```

Мы их будем вызывать в main

## Абстракция над библиотекой

Для удобства я решил обернуть библиотеку в `MyPlugin` тип. Он будет содержать
загруженную библиотеку `Library` и `Symbol`'s на ее функции. Еще, для удобства,
я обернул вызов функций библиотеки в методы имплементации для `MyPlugin`

```rust
use libloading::Library;
use ouroboros::self_referencing;

[self_referencing]
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
```

## Результат

Посмотрим ня нашу main функцию

```rust
fn main() {
    // Загрузка нашего плагина
    let my_plugin = MyPlugin::load("target/debug/libmylib.so").unwrap();

    // Печатаем 'Hello'
    my_plugin.hello();

    // Получаем u32 число
    let val = my_plugin.ret_val();

    println!("val: {val:?}")
}
```

```sh
➜  libload-example git:(main) ✗ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/libload-example`
Hello
val: Some(155)
```
