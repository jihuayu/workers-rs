use worker_macros::event;

#[event(email, respond_with_errors)]
async fn bad_email_flags(_msg: (), _env: (), _ctx: ()) -> Result<(), ()> {
    Ok(())
}

fn main() {}
