mod closures;
mod errors;
mod generics;
pub mod grep;
mod iterators;
mod lifetimes;
mod message_passing;
pub mod misc;
mod object_oriented;
mod shared_state;
mod smart_pointers;
mod smart_pointers_2;
mod threads;
mod traits;

pub fn run() {
    object_oriented::run();
    shared_state::run();
    message_passing::run();
    threads::run();
    smart_pointers_2::run();
    smart_pointers::run();
    iterators::run();
    closures::run();
    grep::run();
    lifetimes::run();
    traits::run();
    generics::run();
    errors::run();
    misc::run();
}
