use worker_macros::event;

#[event(start)]
fn bad_start_signature(_env: ()) {
}

fn main() {}
