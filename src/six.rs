use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Index, IndexMut};
use std::collections::VecDeque;

type SharedSpaceObject =  Rc<RefCell<SpaceObject>>;

#[derive(Debug, PartialEq, Eq)]
struct SpaceObject {
    name: String,
    children: Vec<SharedSpaceObject>,
    parent: Option<SharedSpaceObject>,
    orbit_count: u64,
}

#[derive(Debug)]
struct Galaxy {
    objects: Vec<SharedSpaceObject>,
}

impl SpaceObject {
    fn new(name: String) -> Self {
        SpaceObject{
            name,
            children: Vec::new(),
            parent: None,
            orbit_count: 0,
        }
    }

    fn add_child(&mut self, child: SharedSpaceObject) {
        if !self.children.iter().any(|x| x.borrow().name == child.borrow().name) {
            self.children.push(child);
        }
    }
}

impl Galaxy {
    fn new() -> Self {
        Galaxy {objects: Vec::new()}
    }

    fn find_or_insert_object(&mut self, object_name: &str) -> SharedSpaceObject {
        if !self.object_exists(object_name){
            let object = Rc::new(RefCell::new(SpaceObject::new(object_name.to_string())));
            self.objects.push(object);
        }
        if let Some(result) = self.find_object(object_name) {
            return result;
        } else {
            panic!("Failed to fetch object");
        }
    }

    fn object_exists(&self, object_name: &str) -> bool {
        self.objects.iter().any(|x| x.borrow().name == object_name)
    }

    fn find_object(&mut self, object_name: &str) -> Option<SharedSpaceObject> {
        for object in &self.objects {
            if object.borrow().name == object_name {
                return Some(Rc::clone(&object));
            }
        }
        None
    }

    fn hops(&mut self, from: &str, to: &str) -> Option<u64> {
        let from = self.find_object(from).expect("Failed to find origin for transfer");
        let to = self.find_object(to).expect("Failed to find target for transfer");

        //Breadth-first search
        let mut visited = VecDeque::new();
        let mut to_visit = VecDeque::new();
        if let Some(parent) = &from.borrow().parent {
            to_visit.push_back((0, Rc::clone(parent)));
        } else {
            panic!("No parent for orign object");
        }
        
        while let Some((hops, object_rc)) = to_visit.pop_front() {
            let object = object_rc.borrow();
            visited.push_back(Rc::clone(&object_rc));
            //Check if this is the object
            if object_rc == to {
                return Some(hops - 1);
            }
            //handle parent of object if it exists
            if let Some(parent) = &object.parent {
                if !visited.contains(parent) {
                    to_visit.push_back((hops + 1, Rc::clone(parent)));
                }
            }
            for child in &object.children {
                if !visited.contains(child) {
                    to_visit.push_back((hops + 1, Rc::clone(child)));
                }
            }
        }
        None
    }
}

impl Index<usize> for Galaxy {
    type Output = SharedSpaceObject;

    fn index(&self, index: usize) -> &Self::Output {
        &self.objects[index]
    }
}

impl IndexMut<usize> for Galaxy {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.objects[index]
    }
}


pub fn run(input: Vec<String>) {
    let mut galaxy = Galaxy::new();
    for line in &input {
        let object_names: Vec<&str> = line.split(")").collect();
        if object_names.len() != 2 {
            panic!("More than two objects specified in one line");
        }
        let parent = galaxy.find_or_insert_object(object_names[0]);
        let child = galaxy.find_or_insert_object(object_names[1]);

        parent.borrow_mut().add_child(Rc::clone(&child));
        child.borrow_mut().parent = Some(Rc::clone(&parent));
    }
    
    let com = galaxy.find_object("COM").expect("No COM-Object");
    let mut objects_to_handle = Vec::new();
    objects_to_handle.push(Rc::clone(&com));
    while !objects_to_handle.is_empty() {
        let object_rc = objects_to_handle.remove(0);
        let object = object_rc.borrow();
        for child in &object.children {
            child.borrow_mut().orbit_count = object.orbit_count + 1;
            objects_to_handle.push(Rc::clone(&child));
        }
    }
    let count: u64 = galaxy.objects.iter().map(|x| x.borrow().orbit_count).sum();
    let hops = galaxy.hops("YOU", "SAN").unwrap();
    println!("Day6 Task1: OrbitCount: {}, Task2 Hops: {}", count, hops);
}
