use crustini_core::{Buttons, CrustiniApp, Screen};
use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use std::error::Error;

#[derive(Clone, Debug)]
pub struct WindowConfig {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub fps: usize,
    pub scale: WindowScale,
}

impl WindowConfig {
    pub fn new(title: &str, width: usize, height: usize, fps: usize) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            fps,
            scale: WindowScale::Auto,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowScale {
    Auto,
    X1,
    X2,
    X4,
    X8,
}

pub fn run_window<App>(config: WindowConfig) -> Result<(), Box<dyn Error>>
where
    App: CrustiniApp + Default,
{
    let mut app = App::default();
    let mut pixels = vec![0_u8; config.width * config.height];
    let mut frame = vec![0_u32; config.width * config.height];
    let mut paused = false;

    {
        let mut screen = Screen::new(config.width, config.height, &mut pixels);
        app.setup(&mut screen);
    }

    let mut window = Window::new(
        &window_title(&config, paused),
        config.width,
        config.height,
        WindowOptions {
            resize: false,
            scale: config.scale.resolve(config.width, config.height),
            title: true,
            borderless: false,
            ..WindowOptions::default()
        },
    )?;
    window.set_target_fps(config.fps.max(1));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            app = App::default();
            let mut screen = Screen::new(config.width, config.height, &mut pixels);
            app.setup(&mut screen);
        }

        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            paused = !paused;
            window.set_title(&window_title(&config, paused));
        }

        if !paused {
            let input = read_buttons(&window);
            app.update(input);
        }

        {
            let mut screen = Screen::new(config.width, config.height, &mut pixels);
            app.draw(&mut screen);
        }

        copy_grayscale_frame(&pixels, &mut frame);
        window.update_with_buffer(&frame, config.width, config.height)?;
    }

    Ok(())
}

fn read_buttons(window: &Window) -> Buttons {
    let (mouse_x, mouse_y) = window
        .get_mouse_pos(MouseMode::Clamp)
        .map(|(x, y)| (x as i16, y as i16))
        .unwrap_or((0, 0));

    Buttons {
        up: window.is_key_down(Key::Up) || window.is_key_down(Key::W),
        down: window.is_key_down(Key::Down) || window.is_key_down(Key::S),
        left: window.is_key_down(Key::Left) || window.is_key_down(Key::A),
        right: window.is_key_down(Key::Right) || window.is_key_down(Key::D),
        a: window.is_key_down(Key::Space) || window.is_key_down(Key::Z),
        b: window.is_key_down(Key::Enter) || window.is_key_down(Key::X),
        start: window.is_key_down(Key::Enter),
        select: window.is_key_down(Key::Backspace),
        mouse_x,
        mouse_y,
        mouse_down: window.get_mouse_down(MouseButton::Left),
    }
}

fn copy_grayscale_frame(pixels: &[u8], frame: &mut [u32]) {
    for (dst, &pixel) in frame.iter_mut().zip(pixels) {
        let value = pixel as u32;
        *dst = (value << 16) | (value << 8) | value;
    }
}

fn window_title(config: &WindowConfig, paused: bool) -> String {
    let state = if paused { "paused" } else { "running" };
    format!(
        "{} - {}x{} @ {}fps - {} - Esc quit, P pause, R reset",
        config.title, config.width, config.height, config.fps, state
    )
}

impl WindowScale {
    fn resolve(self, width: usize, height: usize) -> Scale {
        match self {
            Self::Auto => auto_scale(width, height),
            Self::X1 => Scale::X1,
            Self::X2 => Scale::X2,
            Self::X4 => Scale::X4,
            Self::X8 => Scale::X8,
        }
    }
}

fn auto_scale(width: usize, height: usize) -> Scale {
    let max_edge = width.max(height);

    if max_edge <= 64 {
        Scale::X8
    } else if max_edge <= 160 {
        Scale::X4
    } else if max_edge <= 320 {
        Scale::X2
    } else {
        Scale::X1
    }
}
