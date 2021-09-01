// Implement 'push_ping', 'push_substract', 'push_print', 'remove'
// and 'process' methods, so the below code works.

enum Message {
    Ping,
    Substract{minuend: i32, subtrahend: i32},
    Print(String),
}

struct MessageQueue {
    queue: Vec<Message>,
}

impl MessageQueue {
    fn new() -> MessageQueue {
        // create an empty MessageQueue
    }

    fn push_ping(&mut self) {
        // add Ping Message to the queue
    }

    fn push_substract(&mut self, minuend: i32, subtrahend: i32) {
        // add Substract Message to the queue
    }

    fn push_print(&mut self, text: &str) {
        // add Print Message to the queue
    }

    fn ping() {
        println!("Pong!");
    }

    fn substract(m: i32, s: i32) {
        println!("{}", m - s);
    }

    fn print_text(s: &str) {
        println!("{}", s);
    }

    fn remove_front(&mut self) {
        // remove first element of queue
    }

    fn process(&self) {
        // iterate over queue and based on message use one of 'MessageQueue::ping',
        // 'MessageQueue::substract' or 'MessageQueue::print_text' method
    }
}

fn main() {
    let mut queue = MessageQueue::new();
    queue.push_ping();
    queue.push_substract(100, 58);
    queue.push_print("Answer to the Ultimate Question of Life, the Universe, and Everything");

    queue.process();
    queue.remove_front();
    queue.process();
}
