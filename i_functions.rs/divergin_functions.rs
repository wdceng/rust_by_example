fn main() {
    fn foo() -> ! {
        panic!("This call never returns.");
    }

    foo();
}
