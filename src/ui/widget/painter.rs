use druid::widget::Painter;
use druid::{Color, Env, KeyOrValue, RenderContext};

use crate::ui::theme::ColorGroup;

pub fn solid<T>(normal: impl Into<KeyOrValue<Color>>) -> Painter<T> {
    let normal_color = normal.into();
    Painter::new(move |ctx, _, env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &normal_color.resolve(env));
    })
}

pub fn solid_reactive<T>(color_group: ColorGroup) -> Painter<T> {
    dyn_solid_reactive(move |_, _| color_group.clone())
}

pub fn dyn_solid_reactive<T>(
    group_fetcher: impl Fn(&T, &Env) -> ColorGroup + 'static,
) -> Painter<T> {
    Painter::new(move |ctx, data, env| {
        let color_group = group_fetcher(data, env);
        let bounds = ctx.size().to_rect();
        let color = if ctx.is_disabled() {
            env.get(&color_group.disabled)
        } else if ctx.is_active() {
            env.get(&color_group.active)
        } else if ctx.is_hot() {
            env.get(&color_group.hover)
        } else {
            env.get(&color_group.normal)
        };
        ctx.fill(bounds, &color);
    })
}
