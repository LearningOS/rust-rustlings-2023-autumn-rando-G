// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.



use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun.clone()));
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun.clone()));
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun.clone()));
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun.clone()));
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 6 references
    jupiter.details();

    // TODO
    let saturn = Planet::Saturn(Rc::new(Sun {}));
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 7 references
    saturn.details();

    // TODO
    let uranus = Planet::Uranus(Rc::new(Sun {}));
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 8 references
    uranus.details();

    // TODO
    let neptune = Planet::Neptune(Rc::new(Sun {}));
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 6);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 4 references

    // TODO
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 3 references

    // TODO
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 2 references

    // TODO
    println!("reference count = {}", Rc::strong_count(&sun.clone())); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 4);
}
