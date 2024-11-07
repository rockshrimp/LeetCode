use std::collections::VecDeque;
struct RecentCounter {
    ping_queue: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self {
            ping_queue: VecDeque::new(),
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        while let Some(v) = self.ping_queue.front(){
            if *v < t - 3000{
                self.ping_queue.pop_front();
            }else{
                break;
            }
        } 
        self.ping_queue.push_back(t);
        self.ping_queue.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */