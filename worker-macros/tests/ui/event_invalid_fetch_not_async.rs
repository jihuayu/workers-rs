use worker_macros::event;

#[event(fetch)]
fn bad_fetch_signature(_req: (), _env: (), _ctx: ()) -> Result<(), ()> {
    Ok(())
}

fn main() {}
