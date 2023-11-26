trait Subject<T: Clone> {
    fn notify_observers(&self, obj: &T);
    fn register_observer(&mut self, observer: Box<dyn Observer<T>>) -> usize;
    fn unregister_observer(&mut self, id: usize);
}

trait Observer<T: Clone> {
    fn on_notify(&self, obj: &T);
}

#[derive(Debug, Clone)]
struct EventObject(usize);

struct SubjectX {
    observers: Vec<(bool, Box<dyn Observer<EventObject>>)>,
}

impl SubjectX {
    fn new() -> SubjectX {
        SubjectX {
            observers: Vec::new(),
        }
    }
}

impl Subject<EventObject> for SubjectX {
    fn notify_observers(&self, obj: &EventObject) {
        for observer in self.observers.iter() {
            if observer.0 {
                observer.1.on_notify(obj);
            }
        }
    }

    fn register_observer(&mut self, observer: Box<dyn Observer<EventObject>>) -> usize {
        self.observers.push((true, observer));
        self.observers.len() - 1
    }

    fn unregister_observer(&mut self, id: usize) {
        self.observers[id].0 = false
    }
}

struct ObserverX(usize);
impl Observer<EventObject> for ObserverX {
    fn on_notify(&self, obj: &EventObject) {
        println!("ObserverX {} Get {:?}", self.0, obj);
    }
}

#[test]
fn main() {
    let mut subject = SubjectX::new();
    subject.register_observer(Box::new(ObserverX(1)));
    subject.register_observer(Box::new(ObserverX(2)));
    subject.register_observer(Box::new(ObserverX(3)));

    subject.notify_observers(&EventObject(100));
    subject.notify_observers(&EventObject(20));
}
