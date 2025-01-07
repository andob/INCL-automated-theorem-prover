use std::thread;
use std::time::{Duration, Instant};
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use prover::codeloc;
use crate::complexity::Complexity;
use crate::csv::ParsedCSVLine;

pub fn visualize_data(csv_lines : &Vec<ParsedCSVLine>, complexity : &Complexity) -> Result<()>
{
    let sdl = sdl2::init().map_err(|msg|anyhow!(msg)).context(codeloc!())?;
    let video_subsystem = sdl.video().map_err(|msg|anyhow!(msg)).context(codeloc!())?;

    let (window_width, window_height) = (1400u32, 900u32);
    let window = video_subsystem.window(complexity.to_onotation_string().as_str(), window_width, window_height).opengl().build().context(codeloc!())?;

    let (opengl_driver_index, _) = sdl2::render::drivers().find_position(|d| d.name=="opengl").context(codeloc!())?;
    let mut canvas = window.into_canvas().index(opengl_driver_index as u32).accelerated().build().context(codeloc!())?;

    let max_input = csv_lines.iter().map(|line| line.input).max().context(codeloc!())?;
    let max_output = csv_lines.iter().map(|line| line.output).max().context(codeloc!())?;

    let scale_x = (window_width as f64) * 0.99 / (max_input as f64);
    let scale_y = (window_height as f64) * 0.99 / (max_output as f64);

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    canvas.set_draw_color(Color::WHITE);

    for csv_line in csv_lines
    {
        let x = (csv_line.input as f64) * scale_x;
        let y = (window_height as f64) - (csv_line.output as f64) * scale_y;
        canvas.fill_rect(Rect::new((x as i32) - 5, (y as i32) - 5, 5, 5))
            .map_err(|msg|anyhow!(msg.clone())).context(codeloc!())?;
    }

    for input in 1..=max_input
    {
        let x1 = (input as f64) * scale_x;
        let x2 = ((input+1) as f64) * scale_x;
        let y1 = (window_height as f64) - complexity.plot(input as f64) * scale_y;
        let y2 = (window_height as f64) - complexity.plot((input+1) as f64) * scale_y;
        canvas.draw_line(Point::new(x1 as i32, y1 as i32), Point::new(x2 as i32, y2 as i32))
            .map_err(|e|anyhow!(e.clone())).context(codeloc!())?;
    }

    canvas.present();

    let mut sdl_event_pump = sdl.event_pump().map_err(|e|anyhow!(e.clone())).context(codeloc!())?;
    for event in sdl_event_pump.wait_iter()
    {
        match event
        {
            Event::KeyUp { keycode: Some(Keycode::Escape), .. } => { return Ok(()); }
            Event::Quit { .. } => { return Ok(()); }
            _ => {}
        }
    }

    return Ok(());
}
