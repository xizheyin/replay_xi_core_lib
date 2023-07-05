fn main() {
    let mut _local2 = xi_core_lib::line_cache_shadow::Builder::new();
    let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local2), false);
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local2),
        13018007295331248553,
        12225348394837191081,
        247,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local2),
        12225488692137780575,
        7884505976495759219,
        101,
    );
    let mut _local9 = xi_core_lib::line_cache_shadow::Builder::build(_local2);
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::partial_invalidate(
        &mut (_local9),
        7810758480380259893,
        7882834714672496640,
        0,
    );
}
