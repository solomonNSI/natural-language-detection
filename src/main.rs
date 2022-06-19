extern crate whatlang;

use whatlang::{detect, Lang, Script};

fn main() {
    let text = std::env::args().nth(1).expect("Please specify a word");

    let info = detect(&*text).unwrap();
    assert_eq!(info.lang(), Lang::Epo);
    assert_eq!(info.script(), Script::Latin);
    assert_eq!(info.confidence(), 1.0);
    assert!(info.is_reliable());
}