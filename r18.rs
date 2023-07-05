fn main() {
    let mut _local2 = xi_core_lib::line_cache_shadow::Builder::new();
    //let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local2), _param2);
    //let _ = xi_core_lib::width_cache::WidthCache::new();
    //let _ = xi_core_lib::whitespace::Indentation::parse_line(_param3);
    let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local2), false);
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local2),
        7886711674397877607,
        6875136508415272311,
        45,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local2),
        7594244560875513343,
        18446743224907034468,
        105,
    );
    let mut _local9 = xi_core_lib::line_cache_shadow::Builder::build(_local2);
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::partial_invalidate(
        &mut (_local9),
        7384038013278119267,
        8388358299936907634,
        64,
    );
}
