fn main() {
    let mut _local0 = xi_core_lib::line_cache_shadow::Builder::new();

    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        3256716266674222128,
        3523757637854750438,
        230,
    );

    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        16638239252302099258,
        8896413998432348516,
        58,
    );

    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        8587635943441247341,
        7310582922761366895,
        118,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local0), true);
    let mut _local11 = xi_core_lib::line_cache_shadow::Builder::build(_local0);
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::edit(
        &mut (_local11),
        6878244986415899743,
        8027208270804645229,
        11127959463491891809,
    );
}
