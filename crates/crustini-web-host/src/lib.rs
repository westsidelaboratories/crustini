use crustini_core::{Buttons, CrustiniApp, Screen};
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

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
    let page = PageConfig {
        title: config.title,
        width: config.width,
        height: config.height,
        fps: config.fps,
        scale: config.scale.resolve(config.width, config.height),
    };
    let width = page.width;
    let height = page.height;
    let fps = page.fps;
    let state = Arc::new(Mutex::new(HostState {
        pixels: vec![0_u8; width * height],
        input: Buttons::default(),
        paused: false,
        reset_requested: false,
    }));
    let running = Arc::new(AtomicBool::new(true));
    let listener = TcpListener::bind("127.0.0.1:0")?;
    listener.set_nonblocking(true)?;
    let url = format!("http://{}", listener.local_addr()?);

    start_http_host(listener, Arc::clone(&state), Arc::clone(&running), page);
    open_preview_url(&url);
    println!("preview: {}", url);

    let mut app = App::default();
    {
        let mut state = state.lock().unwrap();
        let mut screen = Screen::new(width, height, &mut state.pixels);
        app.setup(&mut screen);
    }

    let frame_time = Duration::from_secs_f64(1.0 / fps.max(1) as f64);
    while running.load(Ordering::Relaxed) {
        let started = Instant::now();
        let (input, paused, reset_requested) = {
            let mut state = state.lock().unwrap();
            let input = state.input;
            let paused = state.paused;
            let reset_requested = state.reset_requested;
            state.reset_requested = false;
            (input, paused, reset_requested)
        };

        if reset_requested {
            app = App::default();
            let mut state = state.lock().unwrap();
            let mut screen = Screen::new(width, height, &mut state.pixels);
            app.setup(&mut screen);
        }

        if !paused {
            app.update(input);
        }
        {
            let mut state = state.lock().unwrap();
            let mut screen = Screen::new(width, height, &mut state.pixels);
            app.draw(&mut screen);
        }

        let elapsed = started.elapsed();
        if elapsed < frame_time {
            thread::sleep(frame_time - elapsed);
        }
    }

    Ok(())
}

struct HostState {
    pixels: Vec<u8>,
    input: Buttons,
    paused: bool,
    reset_requested: bool,
}

#[derive(Clone, Debug)]
struct PageConfig {
    title: String,
    width: usize,
    height: usize,
    fps: usize,
    scale: usize,
}

fn start_http_host(
    listener: TcpListener,
    state: Arc<Mutex<HostState>>,
    running: Arc<AtomicBool>,
    page: PageConfig,
) {
    thread::spawn(move || {
        while running.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((stream, _)) => {
                    handle_stream(stream, &state, &running, &page);
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(4));
                }
                Err(_) => break,
            }
        }
    });
}

fn handle_stream(
    mut stream: TcpStream,
    state: &Arc<Mutex<HostState>>,
    running: &Arc<AtomicBool>,
    page: &PageConfig,
) {
    let mut buf = [0_u8; 2048];
    let Ok(n) = stream.read(&mut buf) else {
        return;
    };
    let request = String::from_utf8_lossy(&buf[..n]);
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    if path == "/" {
        respond_html(&mut stream, page);
    } else if path == "/frame" {
        let pixels = state.lock().unwrap().pixels.clone();
        respond_bytes(&mut stream, "application/octet-stream", &pixels);
    } else if let Some(query) = path.strip_prefix("/input?") {
        state.lock().unwrap().input = parse_input(query);
        respond_text(&mut stream, "ok");
    } else if path == "/reset" {
        state.lock().unwrap().reset_requested = true;
        respond_text(&mut stream, "ok");
    } else if let Some(query) = path.strip_prefix("/pause?") {
        state.lock().unwrap().paused = query.contains("value=1");
        respond_text(&mut stream, "ok");
    } else if path == "/quit" {
        running.store(false, Ordering::Relaxed);
        respond_text(&mut stream, "ok");
    } else {
        respond_not_found(&mut stream);
    }
}

fn parse_input(query: &str) -> Buttons {
    let mut input = Buttons::default();

    for pair in query.split('&') {
        let Some((key, value)) = pair.split_once('=') else {
            continue;
        };
        let on = value == "1" || value == "true";

        match key {
            "up" => input.up = on,
            "down" => input.down = on,
            "left" => input.left = on,
            "right" => input.right = on,
            "a" => input.a = on,
            "b" => input.b = on,
            "start" => input.start = on,
            "select" => input.select = on,
            "mouse_down" => input.mouse_down = on,
            "mouse_x" => input.mouse_x = value.parse().unwrap_or(0),
            "mouse_y" => input.mouse_y = value.parse().unwrap_or(0),
            _ => {}
        }
    }

    input
}

fn respond_html(stream: &mut TcpStream, page: &PageConfig) {
    let html = preview_html(&page.title, page.width, page.height, page.fps, page.scale);
    respond_bytes(stream, "text/html; charset=utf-8", html.as_bytes());
}

fn respond_text(stream: &mut TcpStream, body: &str) {
    respond_bytes(stream, "text/plain; charset=utf-8", body.as_bytes());
}

fn respond_not_found(stream: &mut TcpStream) {
    let body = b"not found";
    let _ = write!(
        stream,
        "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(body);
}

fn respond_bytes(stream: &mut TcpStream, content_type: &str, body: &[u8]) {
    let _ = write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\nCache-Control: no-store\r\n\r\n",
        body.len(),
        content_type
    );
    let _ = stream.write_all(body);
}

fn preview_html(title: &str, width: usize, height: usize, fps: usize, scale: usize) -> String {
    let title = escape_html(title);
    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<title>{title}</title>
<style>
:root{{color-scheme:dark}}
*{{box-sizing:border-box}}
html,body{{margin:0;width:100%;height:100%;background:#111;color:#ddd;font-family:system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif;overflow:hidden}}
body{{display:grid;grid-template-rows:auto 1fr;background:linear-gradient(#181818,#0f0f0f)}}
.shell{{height:44px;display:flex;align-items:center;gap:12px;padding:0 12px;border-bottom:1px solid #2b2b2b;background:#1d1d1d;color:#e8e8e8;user-select:none}}
.title{{font-weight:650;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}}
.meta{{color:#9f9f9f;font-size:12px;white-space:nowrap}}
.spacer{{flex:1}}
.controls{{display:flex;gap:6px}}
button{{height:28px;border:1px solid #3a3a3a;border-radius:6px;background:#262626;color:#eee;padding:0 10px;font:inherit;font-size:12px;cursor:pointer}}
button:hover{{background:#303030;border-color:#555}}
.stage{{display:grid;place-items:center;min-width:0;min-height:0;padding:18px}}
.screen-wrap{{display:grid;gap:10px;justify-items:center}}
canvas{{image-rendering:pixelated;background:#000;box-shadow:0 0 0 1px #333,0 18px 50px rgba(0,0,0,.35);outline:none}}
.hint{{color:#8f8f8f;font-size:12px}}
.stopped{{display:none;color:#f0c36a;font-size:12px}}
body.done .stopped{{display:block}}
body.done .hint{{display:none}}
</style>
</head>
<body>
<header class="shell">
  <div class="title">{title}</div>
  <div class="meta">{width}x{height} · {fps}fps · {scale}x</div>
  <div class="spacer"></div>
  <div class="controls">
    <button id="pause">Pause</button>
    <button id="reset">Reset</button>
    <button id="quit">Quit</button>
  </div>
</header>
<main class="stage">
  <div class="screen-wrap">
    <canvas id="screen" width="{width}" height="{height}"></canvas>
    <div class="hint">Click the app, use arrows or WASD, Space/Z, Enter/X. Esc quits.</div>
    <div class="stopped">Preview stopped. Close this tab.</div>
  </div>
</main>
<script>
const width = {width};
const height = {height};
const scale = {scale};
const canvas = document.getElementById("screen");
const pauseButton = document.getElementById("pause");
const resetButton = document.getElementById("reset");
const quitButton = document.getElementById("quit");
const ctx = canvas.getContext("2d");
const image = ctx.createImageData(width, height);
const keys = new Set();
let mouseX = 0;
let mouseY = 0;
let mouseDown = false;
let paused = false;
let stopped = false;

canvas.style.width = `${{width * scale}}px`;
canvas.style.height = `${{height * scale}}px`;
canvas.tabIndex = 0;
canvas.focus();

addEventListener("keydown", event => {{
  keys.add(event.key.toLowerCase());
  if (event.key === "Escape") quit();
  if (["arrowup","arrowdown","arrowleft","arrowright"," "].includes(event.key.toLowerCase())) event.preventDefault();
}});
addEventListener("keyup", event => keys.delete(event.key.toLowerCase()));
canvas.addEventListener("mousemove", event => {{
  const rect = canvas.getBoundingClientRect();
  mouseX = Math.max(0, Math.min(width - 1, Math.floor((event.clientX - rect.left) * width / rect.width)));
  mouseY = Math.max(0, Math.min(height - 1, Math.floor((event.clientY - rect.top) * height / rect.height)));
}});
canvas.addEventListener("mousedown", () => mouseDown = true);
canvas.addEventListener("mouseup", () => mouseDown = false);
canvas.addEventListener("mouseleave", () => mouseDown = false);
addEventListener("beforeunload", () => navigator.sendBeacon("/quit"));
pauseButton.addEventListener("click", async () => {{
  paused = !paused;
  pauseButton.textContent = paused ? "Resume" : "Pause";
  await fetch(`/pause?value=${{paused ? "1" : "0"}}`).catch(() => {{}});
  canvas.focus();
}});
resetButton.addEventListener("click", async () => {{
  await fetch("/reset").catch(() => {{}});
  canvas.focus();
}});
quitButton.addEventListener("click", quit);

function has(...names) {{
  return names.some(name => keys.has(name));
}}

async function quit() {{
  stopped = true;
  document.body.classList.add("done");
  await fetch("/quit").catch(() => {{}});
}}

async function sendInput() {{
  if (stopped) return;
  const params = new URLSearchParams({{
    up: has("arrowup", "w") ? "1" : "0",
    down: has("arrowdown", "s") ? "1" : "0",
    left: has("arrowleft", "a") ? "1" : "0",
    right: has("arrowright", "d") ? "1" : "0",
    a: has(" ", "z") ? "1" : "0",
    b: has("enter", "x") ? "1" : "0",
    start: has("enter") ? "1" : "0",
    select: has("backspace") ? "1" : "0",
    mouse_x: String(mouseX),
    mouse_y: String(mouseY),
    mouse_down: mouseDown ? "1" : "0",
  }});
  await fetch(`/input?${{params}}`).catch(() => {{}});
}}

async function draw() {{
  if (stopped) return;
  await sendInput();
  const response = await fetch("/frame").catch(() => null);
  if (!response || !response.ok) {{
    stopped = true;
    document.body.classList.add("done");
    return;
  }}
  const pixels = new Uint8Array(await response.arrayBuffer());
  for (let i = 0; i < pixels.length; i++) {{
    const value = pixels[i];
    const j = i * 4;
    image.data[j] = value;
    image.data[j + 1] = value;
    image.data[j + 2] = value;
    image.data[j + 3] = 255;
  }}
  ctx.putImageData(image, 0, 0);
  requestAnimationFrame(draw);
}}

draw();
</script>
</body>
</html>"#
    )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

impl WindowScale {
    fn resolve(self, width: usize, height: usize) -> usize {
        match self {
            Self::Auto => {
                let edge = width.max(height);
                if edge <= 64 {
                    8
                } else if edge <= 160 {
                    4
                } else if edge <= 320 {
                    2
                } else {
                    1
                }
            }
            Self::X1 => 1,
            Self::X2 => 2,
            Self::X4 => 4,
            Self::X8 => 8,
        }
    }
}

fn open_preview_url(url: &str) {
    let result = if cfg!(target_os = "macos") {
        Command::new("open").arg(url).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", url]).status()
    } else {
        Command::new("xdg-open").arg(url).status()
    };

    if result.is_err() {
        eprintln!("open this URL to preview: {}", url);
    }
}
