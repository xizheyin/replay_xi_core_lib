fn main() {
    let mut _local2 = xi_core_lib::line_cache_shadow::Builder::new();
    let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local2), true);
    let _ =
        xi_core_lib::line_cache_shadow::Builder::add_span(&mut (_local2), 0, 236166474025674, 202);
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local2),
        14612714913291487946,
        14612714913291487946,
        116,
    );
    let mut _local9 = xi_core_lib::line_cache_shadow::Builder::build(_local2);
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::partial_invalidate(
        &mut (_local9),
        6881500811490062180,
        7594891270602910821,
        114,
    );
}
