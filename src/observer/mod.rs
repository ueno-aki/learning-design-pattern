trait EventManager<T> {
    fn notify_observers(&self, obj: &T);
    fn subscribe(&mut self, observer: Box<dyn Observer<T>>) -> usize;
    fn unsubscribe(&mut self, id: usize);
}

trait Observer<T> {
    fn on_notify(&self, obj: &T);
}

#[derive(Debug)]
struct RandomNumber(usize);

struct RandomNumEvent {
    observers: Vec<(bool, Box<dyn Observer<RandomNumber>>)>,
}

impl RandomNumEvent {
    fn new() -> RandomNumEvent {
        RandomNumEvent {
            observers: Vec::new(),
        }
    }
    fn execute(&self) {
        let num: usize = rand::random();
        self.notify_observers(&RandomNumber(num))
    }
}

impl EventManager<RandomNumber> for RandomNumEvent {
    fn notify_observers(&self, obj: &RandomNumber) {
        for observer in self.observers.iter() {
            if observer.0 {
                observer.1.on_notify(obj);
            }
        }
    }

    fn subscribe(&mut self, observer: Box<dyn Observer<RandomNumber>>) -> usize {
        self.observers.push((true, observer));
        self.observers.len() - 1
    }

    fn unsubscribe(&mut self, id: usize) {
        self.observers[id].0 = false
    }
}

struct BitObserver;
impl Observer<RandomNumber> for BitObserver {
    fn on_notify(&self, obj: &RandomNumber) {
        println!("BitObserver Get 0b{:b}", obj.0);
    }
}
struct HexObserver;
impl Observer<RandomNumber> for HexObserver {
    fn on_notify(&self, obj: &RandomNumber) {
        println!("HexObserver Get 0x{:x}", obj.0);
    }
}

#[test]
fn main() {
    let mut subject = RandomNumEvent::new();
    let bit_observer = subject.subscribe(Box::new(BitObserver));
    let _ = subject.subscribe(Box::new(HexObserver));

    subject.execute();

    subject.unsubscribe(bit_observer);
    subject.execute();
}
