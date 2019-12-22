pub enum Direction {
    Up(u32),
    Right(u32),
    Down(u32),
    Left(u32)
}

type Point = (i32, i32);
type LineSegment = (Point, Point);

pub fn find_intersection_point(wire_1: &Vec<Direction>, wire_2: &Vec<Direction>) -> Option<Point> {
    let a = gen_lines(wire_1);
    let b = gen_lines(wire_2);

    let mut min_point: Option<Point> = None;

    a.iter().for_each(|segment_a| {
        b.iter().for_each(|segment_b| {
            match intersects(segment_a, segment_b) {
                Some(point) => {
                    match min_point {
                        Some(existing_min) => {
                            if (manhattan_distance(&point, &(0, 0)) < manhattan_distance(&existing_min, &(0, 0))) {
                                min_point = Some(point);
                            }
                        },
                        None => min_point = Some(point)
                    }
                },
                None => ()
            }
        });
    });

    min_point
}

/**
 * http://www.cs.swan.ac.uk/~cssimon/line_intersection.html
 */
fn intersects(a: &LineSegment, b: &LineSegment) -> Option<Point> {
    let ((x1, y1), (x2, y2)) = a;
    let ((x3, y3), (x4, y4)) = b;

    let ta = ((y3 - y4) * (x1 - x3) + (x4 - x3) * (y1 - y3)) as f32 /
             ((x4 - x3) * (y1 - y2) - (x1 - x2) * (y4 - y3)) as f32;
    let tb = ((y1 - y2) * (x1 - x3) + (x2 - x1) * (y1 - y3)) as f32 /
             ((x4 - x3) * (y1 - y2) - (x1 - x2) * (y4 - y3)) as f32;

    if ta >= 0.0 && ta <= 1.0 && tb >= 0.0 && tb <= 1.0 {
        Some((x1 + (ta * (x2 - x1) as f32) as i32, y1 + (ta * (y2 - y1) as f32) as i32))
    } else {
         None
     }
}

fn gen_lines(dirs: &Vec<Direction>) -> Vec<LineSegment> {
    let origin: Point = (0, 0);
    dirs.iter().fold(Vec::new(), |list, dir| {
        let new_line = match list.last() {
            Some((_, prev)) => (*prev, next_point(&prev, &dir)),
            None => (origin, next_point(&origin, &dir))
        };

        [list, vec![new_line]].concat()
    })
}

fn gen_points(wire: &Vec<Direction>) -> Vec<Point> {
    let origin: Point = (0, 0);

    wire.iter().fold(vec![origin], |list, dir| {
        let last = list.last().unwrap();
        let next = next_point(last, &dir);

        [list, vec![next]].concat()
    })
}

fn next_point(start: &Point, delta: &Direction) -> Point {
    match delta {
        Direction::Up(dy) => (start.0, start.1 + *dy as i32),
        Direction::Right(dx) => (start.0 + *dx as i32, start.1),
        Direction::Down(dy) => (start.0, start.1 - *dy as i32),
        Direction::Left(dx) => (start.0 - *dx as i32, start.1),
    }
}

fn manhattan_distance(a: &Point, b: &Point) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Direction::*;

    #[test]
    fn test_intersection() {
        let tests: Vec<(LineSegment, LineSegment, Option<Point>)> = vec![
            (((10, 5), (20, 5)), ((13, 2), (13, 10)), Some((13, 5))),
            (((8, 5),  (3, 5)),  ((6, 7),  (6, 3)),   Some((6, 5))),
            (((0, 0),  (10, 0)), ((0, -2), (-1, -2)), None)
        ];

        tests.iter().for_each(|&(line_1, line_2, expected)| {
            assert_eq!(intersects(&line_1, &line_2), expected);
        });
    }

    #[test]
    fn test_problem() {
        let wire_a = vec![Right(8), Up(5), Left(5), Down(3)];
        let wire_b = vec![Up(7), Right(6), Down(4), Left(4)];

        let point = find_intersection_point(&wire_a, &wire_b);
        assert_eq!(point, Some((3, 3)));
    }
}

