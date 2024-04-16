#![feature(trace_macros)]

macro_rules! each_tt {
    () => {};
    ( $_tt:tt $($rest:tt)* ) => {
        each_tt!( $($rest)* );
    };
}

fn main() {
    trace_macros!(true);
    each_tt!(aaa bbb ccc);
    trace_macros!(false);
}
