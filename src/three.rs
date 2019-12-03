
#[derive(Debug)]
struct Interval {
    low: i64,
    high: i64
}

#[derive(Debug)]
struct Line {
    x: Interval,
    y: Interval,
}

impl Interval {
    fn new(a: i64, b: i64) -> Self {
        Self {
            low: a.min(b),
            high: a.max(b),
        }
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        if self.high < other.low || self.low > other.high {
            return None;
        }
        Some(Interval {
            low: self.low.max(other.low),
            high: self.high.min(other.high),
        })
    }
}

impl Line {
    fn new(input: &str) -> Vec<Self> {
        let instructions: Vec<&str> = input.trim().split(",").collect();

        let mut lines = Vec::new();
        let mut start_point = (0, 0);
        for instruction in &instructions {
            let (line, point) = Self::line_from_instruction(instruction, start_point);
            start_point = point;
            lines.push(line);
        }
        lines
    }

    fn line_from_instruction(instruction: &str, start: (i64, i64)) -> (Self, (i64, i64)) {
        let code = &instruction[0..1];
        let length: i64 = instruction[1..].parse().unwrap();
        let mut up = 0;
        let mut right = 0;
        match code {
            "U" => up = length,
            "D" => up = -length,
            "R" => right = length,
            "L" => right = -length,
            _ => panic!("Invalid instruction"),
        }
        let line = Self {
            x: Interval::new(start.0, start.0 + right),
            y: Interval::new(start.1, start.1 + up),
        };
        (line, (start.0 + right, start.1 + up))
    }

    fn intersect(&self, other: &Self) -> Option<Vec<(i64, i64)>> {
        if let Some(x_intersect) = self.x.intersect(&other.x) {
            if let Some(y_intersect) = self.y.intersect(&other.y) {
                let mut points = Vec::new();
                for x in x_intersect.low..=x_intersect.high {
                    for y in y_intersect.low..=y_intersect.high {
                        points.push((x,y));
                    }
                }
                return Some(points);
            }
        }
        None
    }
}


fn manhattan_dist(point: &(i64, i64)) -> i64 {
    return point.0.abs() + point.1.abs();
}

pub fn run(input: Vec<String>) {
    if input.len() != 2 {
        panic!("More than two lines!");
    }
    let lines_1 = Line::new(&input[0]);
    let lines_2 = Line::new(&input[1]);

    let mut intersection_points: Vec<(i64, i64)> = Vec::new();
    //Get intersections
    for line_1 in &lines_1 {
        for line_2 in &lines_2 {
            if let Some(mut points) = line_1.intersect(line_2) {
                intersection_points.append(&mut points);
            }
        }
    }
    //Remove (0,0) Intersection
    intersection_points.retain(|&x| x != (0,0));
        
    let &closest_intersection = intersection_points.iter().min_by(|x, y| manhattan_dist(x).cmp(&manhattan_dist(y))).unwrap();
    println!("Day3 Part1: Closest intersection: ({},{}) Dist: {}", closest_intersection.0, closest_intersection.1, manhattan_dist(&closest_intersection));

}
