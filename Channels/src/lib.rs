use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl <T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender{
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Sender<T>{
    pub fn send(&mut self, t: T){
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);

        //we need to drop the lock after writing

        drop(queue);

        //we also need to notify the reciever who might we waiting for a response
        self.inner.available.notify_one();
    }
}

impl<T> Receiver<T>{
    pub fn recv(&mut self) -> T{
        let mut queue = self.inner.queue.lock().unwrap();
        loop{
            match queue.pop_front(){
                Some(value) => return value,
                //if we don't have anything to recieve rn, we wait for the sender to send something and notify us
                None => {
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
        
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>){
    let inner = Inner{
        queue: Mutex::new(VecDeque::new()),
        available: Condvar::new(),
    };
    let inner = Arc::new(inner);
    (Sender { inner: inner.clone() }, Receiver { inner: inner.clone()})
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_one(){
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }

    #[test]
    fn send_many(){
        let (mut tx, mut rx) = channel();
        tx.send(13);
        tx.send(42);
        tx.send(23);

        let x = rx.recv(); //13
        assert_eq!(rx.recv(), 42);
    }

    // #[test]
    // fn waiting(){
    //     let (mut tx, mut rx) = channel();
    //     let a = rx.recv();
    //     tx.send(19);
    //     assert_eq!(a, 19);
    // }
}