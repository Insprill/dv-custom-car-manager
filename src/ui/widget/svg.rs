use druid::widget::Svg;
use druid::widget::SvgData;
use druid::widget::Widget;
use druid::Data;
use log::error;

pub fn svg<T: Data>(svg_data: &str) -> impl Widget<T> {
    Svg::new(svg_data.parse::<SvgData>().unwrap_or_else(|err| {
        error!("Failed to load svg: {:?}", err);
        SvgData::default()
    }))
}
