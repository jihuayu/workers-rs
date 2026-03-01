use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
    time::Duration,
};

use wasm_bindgen::{prelude::Closure, JsCast};

fn timeout_to_clear(awoken: bool, timeout_id: Option<i32>) -> Option<i32> {
    if awoken {
        None
    } else {
        timeout_id
    }
}

/// A [Future] for asynchronously waiting.
///
/// # Example:
/// ```rust,ignore
/// use std::time::Duration;
/// use worker::Delay;
///
/// let duration = Duration::from_millis(1000);
///
/// // Waits a second
/// Delay::from(duration).await;
/// ```
#[derive(Debug)]
#[pin_project::pin_project(PinnedDrop)]
pub struct Delay {
    inner: Duration,
    closure: Option<Closure<dyn FnMut()>>,
    timeout_id: Option<i32>,
    awoken: Rc<Cell<bool>>,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if !this.awoken.get() {
            if this.closure.is_none() {
                let awoken = this.awoken.clone();
                let callback_ref = this.closure.get_or_insert_with(move || {
                    let waker = cx.waker().clone();
                    let wake = Box::new(move || {
                        waker.wake_by_ref();
                        awoken.set(true);
                    });

                    Closure::wrap_assert_unwind_safe(wake as _)
                });

                // Then get that closure back and pass it to setTimeout so we can get woken up later.
                let global: web_sys::WorkerGlobalScope = js_sys::global().unchecked_into();
                let timeout_id = global
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        callback_ref.as_ref().unchecked_ref::<js_sys::Function>(),
                        this.inner.as_millis() as i32,
                    )
                    .unwrap();
                *this.timeout_id = Some(timeout_id);
            }

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

impl From<Duration> for Delay {
    fn from(inner: Duration) -> Self {
        Self {
            inner,
            closure: None,
            timeout_id: None,
            awoken: Rc::new(Cell::default()),
        }
    }
}

/// SAFETY: If, for whatever reason, the delay is dropped before the future is ready JS will invoke
/// a dropped future causing memory safety issues. To avoid this we will just clean up the timeout
/// if we drop the delay, cancelling the timeout.
#[pin_project::pinned_drop]
impl PinnedDrop for Delay {
    fn drop(self: Pin<&'_ mut Self>) {
        let this = self.project();

        if let Some(id) = timeout_to_clear(this.awoken.get(), *this.timeout_id) {
            let global: web_sys::WorkerGlobalScope = js_sys::global().unchecked_into();
            global.clear_timeout_with_handle(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::timeout_to_clear;

    #[test]
    fn clears_timeout_when_not_awoken() {
        assert_eq!(timeout_to_clear(false, Some(42)), Some(42));
    }

    #[test]
    fn does_not_clear_timeout_when_awoken() {
        assert_eq!(timeout_to_clear(true, Some(42)), None);
    }

    #[test]
    fn does_not_clear_without_timeout_id() {
        assert_eq!(timeout_to_clear(false, None), None);
    }
}
