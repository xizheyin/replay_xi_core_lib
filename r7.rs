fn main() {
    let _local10 = xi_core_lib::selection::SelRegion::caret(6872316419617283935);

    let mut _local12 = xi_core_lib::selection::Selection::new_simple(_local10);
    let _local13 = xi_core_lib::selection::SelRegion::caret(6872316419617283935);
    let _ = xi_core_lib::selection::Selection::add_range_distinct(&mut (_local12), _local13);
}
