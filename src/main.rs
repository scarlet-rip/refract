/* use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.key(Key::Meta, Press).unwrap();
    enigo.raw(24, Click).unwrap();
    enigo.key(Key::Meta, Release).unwrap();
} */

use sens_matcher_linux::test::test;

fn main() {
    test().unwrap();
}
