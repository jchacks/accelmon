use ratatui::{
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
};

fn memory_barchart(temperatures: &[u8]) -> BarChart {
    let bars: Vec<Bar> = temperatures
        .iter()
        .enumerate()
        .map(|(hour, value)| bar(hour, value))
        .collect();
    let title = Line::from("Weather (Vertical)").centered();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(Block::new().title(title))
        .bar_width(5)
}

fn bar(hour: usize, temperature: &u8) -> Bar {
    Bar::default()
        .value(u64::from(*temperature))
        .label(Line::from(format!("{hour:>02}:00")))
        .text_value(format!("{temperature:>3}°"))
}
