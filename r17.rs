fn main() {
    let mut _local2 = xi_core_lib::line_cache_shadow::Builder::new();
    //let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local2), _param2);
    //let _ = xi_core_lib::width_cache::WidthCache::new();
    //let _ = xi_core_lib::whitespace::Indentation::parse_line(_param3);
    let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local2), false);
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local2),
        18374686958399612279,
        6875136508415272301,
        101,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local2),
        8388235780945831017,
        8371189371208035699,
        58,
    );
    let mut _local9 = xi_core_lib::line_cache_shadow::Builder::build(_local2);
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::partial_invalidate(
        &mut (_local9),
        8863206620071473175,
        3472328296227680304,
        48,
    );
}
