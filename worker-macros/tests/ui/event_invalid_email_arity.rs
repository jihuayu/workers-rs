use worker_macros::event;

#[event(email)]
async fn bad_email_signature(_msg: (), _env: ()) -> Result<(), ()> {
    Ok(())
}

fn main() {}
