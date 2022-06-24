use anyhow::Result;
use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
use plotters::{
    prelude::*,
    style::text_anchor::{HPos, Pos, VPos},
};
use rust_decimal::prelude::ToPrimitive;
use rusty_money::{iso, Money};

use super::gecko::lib::Coin;

const GRAPH_WIDTH: u32 = 1024;
const GRAPH_HEIGHT: u32 = 600;

pub fn get_line_chart(coin: &Coin) -> Result<String> {
    let data = coin.market_data.sparkline_7d.price.clone();
    let (mut high, mut low): (&f64, &f64) = (&data[0], &data[0]);
    let current_time = Utc::now();
    let data_length = data.len() - 1;

    data.iter().for_each(|item| {
        high = if item > high { item } else { high };
        low = if item < low { item } else { low };
    });

    let file_name = format!("{}_{}", coin.id, Local::now().format("%Y%m%d%H%M%S%f"));
    let file_path = format!("{}.png", file_name);

    let root = BitMapBackend::new(&file_path, (GRAPH_WIDTH, GRAPH_HEIGHT)).into_drawing_area();

    root.fill(&RGBColor(30, 30, 30).to_rgba())?;

    let chart_top = high * 1.05;
    let chart_bottom = low / 1.05;
    let chart_y_spec = chart_bottom..chart_top;
    let chart_x_spec = 0..data.len() + 2;
    let chart_color = if data[0] >= data[data.len() - 1] {
        RED
    } else {
        GREEN
    };

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size::<i32>(LabelAreaPosition::Left, 100)
        .set_label_area_size::<i32>(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(chart_x_spec, chart_y_spec)?;

    let bold_line_style = ShapeStyle {
        stroke_width: 1,
        color: RGBColor(55, 55, 55).to_rgba(),
        filled: false,
    };

    let label_style = TextStyle {
        color: RGBColor(255, 255, 255).to_backend_color(),
        font: FontDesc::new(FontFamily::SansSerif, 20.0, FontStyle::Normal),
        pos: Pos {
            h_pos: HPos::Center,
            v_pos: VPos::Top,
        },
    };

    chart
        .configure_mesh()
        .bold_line_style(bold_line_style)
        .disable_x_mesh()
        .axis_style(&RGBColor(30, 30, 30))
        .light_line_style(&RGBColor(30, 30, 30))
        .y_label_formatter(&|y| {
            let y_str = &y.to_string();
            let value = Money::from_str(y_str, iso::USD).unwrap();
            format!("{:0}", value)
        })
        .x_label_formatter(&|x| {
            let hours: i64 = (data_length - x).try_into().unwrap();
            let date = current_time - Duration::hours(hours);
            date.format("%e %b").to_string()
        })
        .y_label_offset::<i32>(-10)
        .x_labels(10)
        .y_labels(8)
        .x_label_style(label_style.clone())
        .y_label_style(label_style)
        .draw()?;

    chart.draw_series(
        AreaSeries::new(
            (0..).zip(data.iter()).map(|(x, y)| (x, *y)),
            0.0,
            &chart_color.mix(0.2),
        )
        .border_style(&chart_color),
    )?;

    root.present()?;

    drop(chart);
    drop(root);
    Ok(file_path)
}

pub fn get_ohlc_chart(data: &Vec<Vec<f64>>, coin: &str) -> Result<String> {
    let (mut high, mut low): (&f64, &f64) = (&data[0][2], &data[0][2]);

    data.iter().for_each(|item| {
        high = if item[2] > *high { &item[2] } else { high };
        low = if item[2] < *low { &item[2] } else { low };
    });

    let (from_date, to_date) = (
        parse_time(data[0][0]) - Duration::hours(8),
        parse_time(data[data.len() - 1][0]) + Duration::hours(8),
    );

    let file_name = format!("{}_{}", coin, Local::now().format("%Y%m%d%H%M%S%f"));
    let file_path = format!("{}.png", file_name);

    let root = BitMapBackend::new(&file_path, (GRAPH_WIDTH, GRAPH_HEIGHT)).into_drawing_area();

    root.fill(&TRANSPARENT)?;

    let chart_top = high * 1.05;
    let chart_bottom = low / 1.05;
    let chart_y_spec = chart_bottom..chart_top;
    let chart_x_spec = from_date..to_date;

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size::<i32>(LabelAreaPosition::Left, 100)
        .set_label_area_size::<i32>(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(chart_x_spec, chart_y_spec)?;

    let bold_line_style = ShapeStyle {
        stroke_width: 1,
        color: RGBColor(55, 55, 55).to_rgba(),
        filled: true,
    };

    let label_style = TextStyle {
        color: RGBColor(255, 255, 255).to_backend_color(),
        font: FontDesc::new(FontFamily::SansSerif, 20.0, FontStyle::Normal),
        pos: Pos {
            h_pos: HPos::Center,
            v_pos: VPos::Top,
        },
    };

    chart
        .configure_mesh()
        .bold_line_style(bold_line_style)
        .disable_x_mesh()
        .y_label_formatter(&|y| {
            let y_str = &y.to_string();
            let value = Money::from_str(y_str, iso::USD).unwrap();
            format!("{:0}", value)
        })
        .x_label_formatter(&|x| x.format("%e %b").to_string())
        .y_label_offset::<i32>(-10)
        .x_labels(10)
        .y_labels(8)
        .x_label_style(label_style.clone())
        .y_label_style(label_style)
        .draw()?;

    chart.draw_series(data.iter().map(|x| {
        CandleStick::new(
            parse_time(x[0]),
            x[1],
            x[2],
            x[3],
            x[4],
            GREEN.filled(),
            RED.filled(),
            15,
        )
    }))?;

    root.present()?;

    drop(chart);
    drop(root);
    Ok(file_path)
}

fn parse_time(timestamp: f64) -> DateTime<Utc> {
    let converted_timestamp = timestamp.to_i64().unwrap() / 1000;
    let naive_date_time = NaiveDateTime::from_timestamp(converted_timestamp, 0);
    DateTime::from_utc(naive_date_time, Utc)
}
