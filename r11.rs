fn main() {
    let mut _local0 = xi_core_lib::line_cache_shadow::Builder::new();

    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        11204954773386174496,
        2314885530818453536,
        44,
    );

    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        3328280407072466278,
        7310016669964137262,
        120,
    );

    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        7277867783359394816,
        1022201495551,
        255,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local0), true);
    let mut _local11 = xi_core_lib::line_cache_shadow::Builder::build(_local0);
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::edit(
        &mut (_local11),
        4208166786336385380,
        4208166786336385380,
        0,
    );
}
