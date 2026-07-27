use michie::memoized;
use std::collections::BTreeMap;
use std::sync::MutexGuard;

struct NotSend(MutexGuard<'static, ()>);

impl Clone for NotSend {
    fn clone(&self) -> Self {
        unreachable!()
    }
}

impl PartialEq for NotSend {
    fn eq(&self, _: &NotSend) -> bool {
        unreachable!()
    }
}

impl Eq for NotSend {}

impl PartialOrd for NotSend {
    fn partial_cmp(&self, _: &NotSend) -> Option<std::cmp::Ordering> {
        unreachable!()
    }
}

impl Ord for NotSend {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        unreachable!()
    }
}

fn generic_in_impl() {
    #[memoized(key_expr = (), store_type = BTreeMap<(), NotSend>)]
    #[allow(unreachable_code)]
    fn f() -> NotSend {
        unreachable!()
    }
}

fn main() {}
