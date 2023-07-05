fn main() {
    let _local1 = xi_core_lib::tabs::test_helpers::new_view_id(3629673002777212283);
    let _local2 = xi_core_lib::tabs::BufferId::new(3618136732211641692);
    let mut _local3 = xi_core_lib::view::View::new(_local1, _local2);
    let _ = xi_core_lib::view::View::is_point_in_selection(&(_local3), 8465415132958195195);
    let _ =
        xi_core_lib::view::View::set_scroll(&mut (_local3), 8465362124439040093, -42351830092437);
}
