use worker_macros::event;

#[event(tail)]
async fn bad_tail_return(_event: (), _env: (), _ctx: ()) {
}

fn main() {}
