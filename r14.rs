fn main() {
    let mut _local0 = xi_core_lib::line_cache_shadow::Builder::new();
    let _ = xi_core_lib::selection::SelRegion::caret(3616443506805273704);
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        8026604645052636380,
        15914838024376868060,
        220,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        15914705587303903337,
        7218479844628460648,
        111,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        7222237163774369644,
        7294278294984290913,
        109,
    );
    let _ = xi_core_lib::line_cache_shadow::Builder::set_dirty(&mut (_local0), false);
    let _ = xi_core_lib::line_cache_shadow::Builder::add_span(
        &mut (_local0),
        5366988285518283264,
        7222215147487964521,
        100,
    );
    let _local8 = xi_core_lib::line_cache_shadow::Builder::build(_local0);
    let mut _local9 = xi_core_lib::line_cache_shadow::RenderPlan::create(
        3256432853930764399,
        7222260253887651692,
        7289774695357183839,
    );
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::iter_with_plan(&(_local8), &(_local9));
    let _ = xi_core_lib::line_cache_shadow::LineCacheShadow::needs_render(&(_local8), &(_local9));
}
