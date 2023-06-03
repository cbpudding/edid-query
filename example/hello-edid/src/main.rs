use raw_window_handle::HasRawDisplayHandle;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let raw_handle = event_loop.raw_display_handle();
    let edids = edid_query::query_edid(raw_handle);
    for edid in edids {
        eprintln!("{edid:?}");
    }
}