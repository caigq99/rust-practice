use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

struct SleepFuture {
    duration: Duration,
    state: Arc<Mutex<State>>,
}
impl SleepFuture {
    fn new(from_secs: Duration) -> Self {
        Self {
            duration: from_secs,
            state: Arc::new(Mutex::new(State {
                waker: None,
                inner_state: InnerState::Init,
            })),
        }
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        match state.inner_state {
            InnerState::Init => {
                state.waker = Some(cx.waker().clone());
                state.inner_state = InnerState::Sleeping;
                let duration = self.duration;
                let state_cloned = Arc::clone(&self.state);
                thread::spawn(move || {
                    thread::sleep(duration);
                    let mut state = state_cloned.lock().unwrap();
                    state.inner_state = InnerState::Done;
                    if let Some(waker) = state.waker.take() {
                        waker.wake();
                    }
                });
                Poll::Pending
            }
            InnerState::Sleeping => Poll::Pending,
            InnerState::Done => Poll::Ready(()),
        }
    }
}
struct State {
    waker: Option<Waker>,
    inner_state: InnerState,
}

enum InnerState {
    Init,
    Sleeping,
    Done,
}

#[tokio::main]
async fn main() {
    println!("开始睡眠");
    SleepFuture::new(Duration::from_secs(3)).await;
    println!("结束睡眠");
}
