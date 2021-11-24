use cpp_gen_in_proc_macro_impl::bar;

#[bar]
fn foo() -> i32 {}

fn main() {
    println!("{}", foo());
}
