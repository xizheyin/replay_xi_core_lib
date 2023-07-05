fn main() {
    let _local2 = xi_core_lib::selection::SelRegion::new(3544668469065756977, 3544668469065756977);

    let mut _local4 = xi_core_lib::selection::Selection::new();
    let _ = xi_core_lib::selection::Selection::add_range_distinct(&mut (_local4), _local2);

    let _ = xi_core_lib::selection::Selection::delete_range(
        &mut (_local4),
        3544668469065756977,
        3544668469065756977,
        false,
    );
}
