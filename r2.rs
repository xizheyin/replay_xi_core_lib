fn main() {
    let mut _local3 = xi_core_lib::selection::Selection::new();
    let _local4 = xi_core_lib::selection::SelRegion::caret(14178497895279628484);
    let _ = xi_core_lib::selection::Selection::add_range_distinct(&mut (_local3), _local4);

    let _ = xi_core_lib::selection::Selection::delete_range(
        &mut (_local3),
        13238251629368031159,
        13238251629574210756,
        true,
    );

    let _ = xi_core_lib::selection::Selection::regions_in_range(
        &(_local3),
        14191339064585084356,
        2645584148269417488,
    );
}
