fn main() {
    let mut _local0 = xi_core_lib::line_cache_shadow::Builder::new();
    let _ = xi_core_lib::selection::SelRegion::caret(3616443506798755940);
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        4212670385648923692,
        8100115550063848805,
        116,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        18410544004307318373,
        6875982032941771105,
        114,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        7020394157875095808,
        28556964542036581,
        119,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local0), false);
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        7594244560875513204,
        7525343705305156709,
        95,
    );
    let _local8 = xi_core_lib::line_cache_shadow::Builder::build(_local0);
    let mut _local9 = xi_core_lib::line_cache_shadow::RenderPlan::create(
        7810695838499956087,
        6875137240868061303,
        2119335244915828224,
    );
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::iter_with_plan(&(_local8), &(_local9));
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::needs_render(&(_local8), &(_local9));
}
