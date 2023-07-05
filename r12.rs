fn main() {
    let mut _local0 = xi_core_lib::line_cache_shadow::Builder::new();

    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(&mut (_local0), 0, 0, 0);

    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(&mut (_local0), 0, 184, 184);

    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        13310591802206107832,
        13294626100008321279,
        0,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local0), true);
    let mut _local11 = xi_core_lib::line_cache_shadow::Builder::build(_local0);
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::edit(
        &mut (_local11),
        1095215546368,
        8587520064465403938,
        0,
    );
}
