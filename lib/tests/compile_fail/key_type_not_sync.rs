use michie::memoized;
use std::cell::Cell;
use std::collections::BTreeMap;

struct NotSync(Cell<()>);

impl PartialEq for NotSync {
    fn eq(&self, _: &NotSync) -> bool {
        unreachable!()
    }
}

impl Eq for NotSync {}

impl PartialOrd for NotSync {
    fn partial_cmp(&self, _: &NotSync) -> Option<std::cmp::Ordering> {
        unreachable!()
    }
}

impl Ord for NotSync {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        unreachable!()
    }
}

fn generic_in_impl() {
    #[memoized(key_expr = input, store_type = BTreeMap<NotSync, ()>)]
    fn f(input: NotSync) -> () {
        ()
    }
}

fn main() {}
