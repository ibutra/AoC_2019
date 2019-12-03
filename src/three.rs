use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

type Distance = i64;

#[derive(Debug)]
struct Line {
    points: HashMap<Point, Distance>,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manhatten_dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Line {
    fn new(input: &str) -> Self {
        let instructions: Vec<&str> = input.trim().split(",").collect();

        let mut point = Point::new(0, 0);
        let mut distance = 0;

        let mut line = Self {
            points: HashMap::new(),
        };
        for instruction in &instructions {
            let (filled_point, filled_distance) =
                line.fill_from_instruction(instruction, point, distance);
            point = filled_point;
            distance = filled_distance;
        }
        line
    }

    fn fill_from_instruction(
        &mut self,
        instruction: &str,
        start: Point,
        distance: i64,
    ) -> (Point, Distance) {
        let code = &instruction[0..1];
        let length: i64 = instruction[1..].parse().unwrap();
        let mut points = Vec::new();
        let end;
        match code {
            "U" => {
                for y in start.y+1..=start.y + length {
                    points.push(Point::new(start.x, y));
                }
                end = Point::new(start.x, start.y + length);
            }
            "D" => {
                for y in (start.y - length..=start.y-1).rev() {
                    points.push(Point::new(start.x, y));
                }
                end = Point::new(start.x, start.y - length);
            }
            "R" => {
                for x in start.x+1..=start.x + length {
                    points.push(Point::new(x, start.y));
                }
                end = Point::new(start.x + length, start.y);
            }
            "L" => {
                for x in (start.x - length..=start.x-1).rev() {
                    points.push(Point::new(x, start.y));
                }
                end = Point::new(start.x - length, start.y);
            }
            _ => panic!("Invalid instruction"),
        };
        let mut distance = distance;
        for point in points {
            distance += 1;
            let _ = self.fill_point(point, distance);
        }

        (end, distance)
    }

    fn fill_point(&mut self, point: Point, distance: Distance) -> Distance {
        //Because distance is monotoniously increasing we can always assume a distance already
        //saved is lower than the one passed to this function
        self.points.entry(point).or_insert(distance).clone()
    }

    fn intersections(&self, other: &Self) -> Vec<Point> {
        let my_points: HashSet<Point> = self.points.keys().map(|x| x.clone()).collect();
        let their_points: HashSet<Point> = other.points.keys().map(|x| x.clone()).collect();
        my_points.intersection(&their_points).map(|x| x.clone()).collect()
    }

    fn distance(&self, point: &Point) -> Option<&Distance> {
        self.points.get(point)
    }

    fn combined_distance(&self, other: &Self, point: &Point) -> Option<Distance> {
        if let Some(my_distance) = self.distance(point) {
            if let Some(other_distance) = other.distance(point) {
                return Some(my_distance + other_distance);
            }
        }
        None
    }
}

// fn print_points(line: &Line) {
//     let mut points = Vec::new();
//     for (key, val) in line.points.iter() {
//         points.push((key, val));
//     }
//     points.sort_by_key(|x| x.1);
//     for point in points {
//         println!("{:?}", point);
//     }
// }

pub fn run(input: Vec<String>) {
    if input.len() != 2 {
        panic!("More than two lines!");
    }
    let line_1 = Line::new(&input[0]);
    let line_2 = Line::new(&input[1]);
    let mut intersections = line_1.intersections(&line_2);
    //remove (0,0)
    intersections.retain(|x| *x != Point::new(0,0));
    
    let closest_manhatten_intersection = intersections.iter().min_by_key(|x| x.manhatten_dist(&Point::new(0,0))).unwrap();
    let closest_distance_intersection = intersections.iter().min_by_key(|x| line_1.combined_distance(&line_2, x).unwrap()).unwrap();
        

    // print_points(&line_1);
    // println!("");
    // print_points(&line_2);

    // for point in &intersections {
    //     println!("{:?}", point);
    // }

    println!("Day3 Part1: ({},{}) Manhatten_Distance: {}, Distance: {}", closest_manhatten_intersection.x, closest_manhatten_intersection.y, closest_manhatten_intersection.manhatten_dist(&Point::new(0,0)), line_1.combined_distance(&line_2, closest_manhatten_intersection).unwrap());
    println!("Day3 Part2: ({},{}) Manhatten_Distance: {}, Distance: {}", closest_distance_intersection.x, closest_distance_intersection.y, closest_distance_intersection.manhatten_dist(&Point::new(0,0)), line_1.combined_distance(&line_2, closest_distance_intersection).unwrap());

}
