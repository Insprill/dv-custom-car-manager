use druid::widget::Painter;
use druid::{Color, KeyOrValue, RenderContext};

pub fn solid<T>(normal: impl Into<KeyOrValue<Color>>) -> Painter<T> {
    let normal_color = normal.into();
    Painter::new(move |ctx, _, env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &normal_color.resolve(env));
    })
}

pub fn solid_reactive<T>(
    normal: impl Into<KeyOrValue<Color>>,
    hover: impl Into<KeyOrValue<Color>>,
    active: impl Into<KeyOrValue<Color>>,
) -> Painter<T> {
    let normal_color = normal.into();
    let hover_color = hover.into();
    let active_color = active.into();
    Painter::new(move |ctx, _, env| {
        let bounds = ctx.size().to_rect();
        let color = if ctx.is_active() {
            active_color.resolve(env)
        } else if ctx.is_hot() {
            hover_color.resolve(env)
        } else {
            normal_color.resolve(env)
        };
        ctx.fill(bounds, &color);
    })
}
