use michie::memoized;
use std::cell::Cell;
use std::collections::HashMap;

struct NotSync;

impl !Sync for NotSync {}

fn generic_in_impl() {
    #[memoized(key_expr = input, store_type = HashMap<NotSync, ()>)]
    fn f(input: Cell<()>) -> () {
        ()
    }
}

fn main() {}

