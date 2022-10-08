use dioxus::prelude::*;

// Does compile
#[inline_props]
fn view1(cx: Scope, _v: Option<u32>) -> Element {
    cx.render(rsx!{ "view1" })
}

// Does compile
#[inline_props]
fn view2(cx: Scope, #[props(!optional)] _v: Option<u32>) -> Element {
    cx.render(rsx!{ "view2" })
}

// Does compile
#[inline_props]
pub fn view3(cx: Scope, _v: Option<u32>) -> Element {
    cx.render(rsx!{ "view3" })
}

// Does not compile!!!
//
// error: expected identifier, found `#`
//   --> src/main.rs:36:25
//    |
// 23 | pub fn view4(cx: Scope, #[props(!optional)] _v: Option<u32>) -> Element {
//    |                         ^ expected identifier
// 
// error: expected identifier
//   --> src/main.rs:36:25
//    |
// 23 | pub fn view4(cx: Scope, #[props(!optional)] _v: Option<u32>) -> Element {
//    |                         ^
//
#[inline_props]
pub fn view4(cx: Scope, #[props(!optional)] _v: Option<u32>) -> Element {
    cx.render(rsx!{ "view4" })
}

fn main() {
    println!("Hello, world!");
}
