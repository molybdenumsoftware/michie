use michie::memoized;
use std::collections::BTreeMap;
use std::sync::MutexGuard;

fn generic_in_impl() {
    #[memoized(key_expr = input, store_type = BTreeMap<MutexGuard<'_, ()>, ()>)]
    fn f(input: MutexGuard<'_, ()>) -> () {
        ()
    }
}

fn main() {}
