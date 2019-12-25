type Vector = (i32, i32);
type LineSegment = (Vector, Vector);

const ORIGIN: Vector = (0, 0);

pub fn run(wire_a_input: &str, wire_b_input: &str) -> Option<u32> {
    let wire_a = gen_lines(&direction::parse_input(wire_a_input));
    let wire_b = gen_lines(&direction::parse_input(wire_b_input));

    match find_intersection_point_pt_2(&wire_a, &wire_b) {
        Some((_, dist)) => Some(dist),
        None => None,
    }
}

fn find_intersection_point_pt_2(
    wire_a: &Vec<LineSegment>,
    wire_b: &Vec<LineSegment>,
) -> Option<(Vector, u32)> {
    let mut min: Option<(Vector, u32)> = None;

    wire_a.iter().fold(0, |distance_a, segment_a| {
        wire_b.iter().fold(0, |distance_b, segment_b| {
            match intersects(segment_a, segment_b) {
                Some(point) => {
                    let dist = distance_a
                        + manhattan_distance(&segment_a.0, &point)
                        + distance_b
                        + manhattan_distance(&segment_b.0, &point);

                    match min {
                        Some((_, min_so_far)) => {
                            if (dist < min_so_far) {
                                min = Some((point, dist));
                            }
                        }

                        None => min = Some((point, dist)),
                    }
                }
                None => (),
            }

            distance_b + manhattan_distance(&segment_b.0, &segment_b.1)
        });

        distance_a + manhattan_distance(&segment_a.0, &segment_a.1)
    });

    min
}

fn find_intersection_point_pt_1(
    a: &Vec<LineSegment>,
    b: &Vec<LineSegment>,
) -> Option<(Vector, u32)> {
    let mut min_point: Option<(Vector, u32)> = None;

    a.iter().for_each(|segment_a| {
        b.iter()
            .for_each(|segment_b| match intersects(segment_a, segment_b) {
                Some(point) => match min_point {
                    Some((_, min_dist)) => {
                        let dist = manhattan_distance(&point, &ORIGIN);
                        if dist < min_dist {
                            min_point = Some((point, dist));
                        }
                    }
                    None => min_point = Some((point, manhattan_distance(&point, &ORIGIN))),
                },
                None => (),
            });
    });

    min_point
}

/**
 * http://www.cs.swan.ac.uk/~cssimon/line_intersection.html
 */
fn intersects(a: &LineSegment, b: &LineSegment) -> Option<Vector> {
    let ((x1, y1), (x2, y2)) = a;
    let ((x3, y3), (x4, y4)) = b;

    let ta = ((y3 - y4) * (x1 - x3) + (x4 - x3) * (y1 - y3)) as f32
        / ((x4 - x3) * (y1 - y2) - (x1 - x2) * (y4 - y3)) as f32;
    let tb = ((y1 - y2) * (x1 - x3) + (x2 - x1) * (y1 - y3)) as f32
        / ((x4 - x3) * (y1 - y2) - (x1 - x2) * (y4 - y3)) as f32;

    if ta >= 0.0 && ta <= 1.0 && tb >= 0.0 && tb <= 1.0 {
        Some((
            x1 + (ta * (x2 - x1) as f32) as i32,
            y1 + (ta * (y2 - y1) as f32) as i32,
        ))
    } else {
        None
    }
}

fn gen_lines(dirs: &Vec<Vector>) -> Vec<LineSegment> {
    dirs.iter().fold(Vec::new(), |list, delta| {
        let new_line = match list.last() {
            Some((_, prev)) => (*prev, next_point(&prev, &delta)),
            None => (ORIGIN, next_point(&ORIGIN, &delta)),
        };

        [list, vec![new_line]].concat()
    })
}

fn next_point(start: &Vector, delta: &Vector) -> Vector {
    return (start.0 + delta.0, start.1 + delta.1);
}

fn manhattan_distance(a: &Vector, b: &Vector) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

mod direction {
    use super::Vector;

    pub fn parse_input(input: &str) -> Vec<Vector> {
        input
            .split(',')
            .map(|chunk| {
                // chunk looks like R111
                // split after the first character
                let (dir, delta_str) = chunk.split_at(1);
                let delta: u32 = delta_str.parse().unwrap();

                match dir {
                    "U" => up(delta),
                    "R" => right(delta),
                    "D" => down(delta),
                    "L" => left(delta),
                    _ => panic!(format!("Unrecognized input {} {}", dir, delta)),
                }
            })
            .collect()
    }

    pub fn up(dy: u32) -> Vector {
        (0, dy as i32)
    }

    pub fn right(dx: u32) -> Vector {
        (dx as i32, 0)
    }

    pub fn down(dy: u32) -> Vector {
        (0, -1 * dy as i32)
    }

    pub fn left(dx: u32) -> Vector {
        (-1 * dx as i32, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::direction::*;
    use super::*;

    #[test]
    fn test_intersection() {
        let tests: Vec<(LineSegment, LineSegment, Option<Vector>)> = vec![
            (((10, 5), (20, 5)), ((13, 2), (13, 10)), Some((13, 5))),
            (((8, 5), (3, 5)), ((6, 7), (6, 3)), Some((6, 5))),
            (((0, 0), (10, 0)), ((0, -2), (-1, -2)), None),
        ];

        tests.iter().for_each(|&(line_1, line_2, expected)| {
            assert_eq!(intersects(&line_1, &line_2), expected);
        });
    }

    // #[test]
    // fn test_pt_1() {
    //     let wire_a = vec![right(8), up(5), left(5), down(3)];
    //     let wire_b = vec![up(7), right(6), down(4), left(4)];

    //     let point = find_intersection_point_pt_1(&wire_a, &wire_b);
    //     assert_eq!(point, Some((3, 3)));
    // }
}
