use cpp_gen_in_proc_macro::bar;

#[bar]
fn foo() -> i32 {}

fn main() {
    println!("{}", foo());
}
