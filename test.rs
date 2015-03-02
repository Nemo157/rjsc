#![feature(no_std)]
#![feature(lang_items)]
#![no_std]

#[lang="phantom_fn"]
trait PhantomFn<A: ?Sized, R: ?Sized = ()> {}

#[lang="sized"]
trait Sized: PhantomFn<Self> {}

#[lang="copy"]
trait Copy: PhantomFn<Self> {}

#[lang="sync"]
trait Sync: PhantomFn<Self> {}


mod console {
    extern {
        pub fn log(s: &str);
    }
}

fn log(s: &str) {
    unsafe { console::log(s); }
}

fn main() {
    log("Hello World!");
}
