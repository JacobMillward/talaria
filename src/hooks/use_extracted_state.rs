use dioxus::prelude::*;

pub fn use_extracted_state<T, F, R>(input: Signal<T>, extract: F) -> Signal<R>
where
    T: Copy + Clone + 'static,
    F: Fn(T) -> R + Copy + Clone + 'static,
{
    let mut state = use_signal(move || extract(*input.read()));
    use_effect(move || {
        state.set(extract(*input.read()));
    });

    state
}