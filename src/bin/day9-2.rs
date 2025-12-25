use AdventOfCode2025::utils::read_lines_into_vec;
use image::{Rgb, RgbImage};
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn draw_grid(red_pixels: &[Point], green_pixels: &[Point]) {
    // Find maximum coordinates
    let mut max_x = 0;
    let mut max_y = 0;

    for p in red_pixels.iter().chain(green_pixels.iter()) {
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }

    // Each cell is 10x10 pixels — +1 so last cell is included
    let width = (max_x + 1) * 10;
    let height = (max_y + 1) * 10;

    let mut image = RgbImage::from_pixel(width as u32, height as u32, Rgb([255, 255, 255]));

    // Helper to draw a 10x10 block
    let mut draw_block = |p: &Point, color: Rgb<u8>| {
        let base_x = p.x * 10;
        let base_y = p.y * 10;

        for dx in 0..10 {
            for dy in 0..10 {
                let x = base_x + dx;
                let y = base_y + dy;
                if x < width && y < height {
                    image.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    };

    for p in red_pixels {
        draw_block(p, Rgb([255, 0, 0]));
    }

    for p in green_pixels {
        draw_block(p, Rgb([0, 255, 0]));
    }

    image.save("output-compressed.png").unwrap();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn area(&self, other: &Point) -> usize {
        return (((self.x as i64 - other.x as i64).abs() + 1)
            * ((self.y as i64 - other.y as i64).abs() + 1)) as usize;
    }
}

fn get_flood_fill_area(
    start: Point,
    boundary: &HashSet<Point>,
    width: usize,
    height: usize,
) -> HashSet<Point> {
    let mut to_visit: Vec<Point> = vec![start];
    let mut visited: HashSet<Point> = HashSet::new();

    let directions: Vec<(i64, i64)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some(current) = to_visit.pop() {
        if boundary.contains(&current) {
            continue;
        }
        println!("Visiting point: {:?}", current);
        visited.insert(current);

        for (dx, dy) in &directions {
            let new_x = current.x as i64 + dx;
            let new_y = current.y as i64 + dy;
            // allow a boundary around for the flood fill to propagate
            if new_x >= -1
                && new_y >= -1
                && new_x < (width as i64) + 1
                && new_y < (height as i64) + 1
            {
                let new_point = Point {
                    x: new_x as usize,
                    y: new_y as usize,
                };
                if !visited.contains(&new_point) && !boundary.contains(&new_point) {
                    to_visit.push(new_point);
                }
            }
        }
    }

    visited
}

fn main() {
    // parse all the red points, figure out all the green points
    // compress the red points into the unique x and y coordinates
    // also put all the green points that fit in the coordinates into compressed coords
    // then, flood fill from the exterior of the compressed red + green points to find all outside points
    // iterate over all the possible rectangles and see if any of their pints are outside. If they are, they are invalid
    // if they are valid, consider their area for max area

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("9.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    let mut corners: Vec<Point> = Vec::new();
    let mut polygon_contour: HashSet<Point> = HashSet::new();
    let mut all_x: HashSet<usize> = HashSet::new();
    let mut all_y: HashSet<usize> = HashSet::new();
    let last_line = lines.last().unwrap();
    let last_split: Vec<&str> = last_line.split(",").collect();
    let mut last_point: Point = Point {
        x: last_split[0].parse().unwrap(),
        y: last_split[1].parse().unwrap(),
    };

    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        let splitted: Vec<&str> = line.split(",").collect();
        let (x, y) = (splitted[0].parse().unwrap(), splitted[1].parse().unwrap());
        let current_point = Point { x: x, y: y };
        all_x.insert(current_point.x);
        all_y.insert(current_point.y);
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }

        if last_point.x != current_point.x {
            // changed in x, horizontal line. Fine to insert duplicates because its a hashset
            for i in min(last_point.x, current_point.x)..=max(current_point.x, last_point.x) {
                //
                polygon_contour.insert(Point {
                    x: i,
                    y: last_point.y,
                });
            }
        } else {
            // changed in y, vertical line
            for i in min(last_point.y, current_point.y)..=max(last_point.y, current_point.y) {
                polygon_contour.insert(Point {
                    x: last_point.x,
                    y: i,
                });
            }
        }
        corners.push(current_point);
        // polygon_contour.insert(current_point);
        last_point = current_point;
    }
    //
    let mut sorted_x: Vec<usize> = all_x.into_iter().collect();
    sorted_x.sort_unstable();
    let mut sorted_y: Vec<usize> = all_y.into_iter().collect();
    sorted_y.sort_unstable();

    let mut x_lookup: HashMap<usize, usize> = HashMap::new();
    let mut y_lookup: HashMap<usize, usize> = HashMap::new();
    for (i, &x) in sorted_x.iter().enumerate() {
        x_lookup.insert(x, i);
    }
    for (i, &y) in sorted_y.iter().enumerate() {
        y_lookup.insert(y, i);
    }

    // convert corners to compressed coords
    let compressed_corners: HashSet<Point> = corners
        .iter()
        .map(|p| Point {
            x: *x_lookup.get(&p.x).unwrap(),
            y: *y_lookup.get(&p.y).unwrap(),
        })
        .collect::<HashSet<Point>>();

    // make a new set that is the polygon contour in compressed coords
    let mut compressed_polygon_contour: HashSet<Point> = HashSet::new();
    for point in &polygon_contour {
        // only insert if in lookups
        if x_lookup.contains_key(&point.x) && y_lookup.contains_key(&point.y) {
            compressed_polygon_contour.insert(Point {
                x: *x_lookup.get(&point.x).unwrap(),
                y: *y_lookup.get(&point.y).unwrap(),
            });
        }
    }

    draw_grid(
        &compressed_corners.iter().cloned().collect::<Vec<Point>>(),
        &compressed_polygon_contour
            .iter()
            .cloned()
            .collect::<Vec<Point>>(),
    );

    //combine the two sets
    let mut combined_boundary: HashSet<Point> = compressed_polygon_contour;
    for point in &compressed_corners {
        combined_boundary.insert(*point);
    }

    let comp_max_x = sorted_x.len();
    let comp_max_y = sorted_y.len();

    // flood fill in compressed space to find the outside points

    // get a set of points outside the contour via flood fill
    let outside_points: HashSet<Point> = get_flood_fill_area(
        Point { x: 0, y: 0 },
        &combined_boundary,
        comp_max_x,
        comp_max_y,
    );

    let mut max_area = 0;
    let unique_tuples: Vec<(Point, Point)> = compressed_corners
        .iter()
        .cloned()
        .tuple_combinations()
        .collect();
    for (point_a, point_b) in unique_tuples {
        let mut contains_outside = false;
        let point_a_real = Point {
            x: sorted_x[point_a.x],
            y: sorted_y[point_a.y],
        };
        let point_b_real = Point {
            x: sorted_x[point_b.x],
            y: sorted_y[point_b.y],
        };
        println!(
            "Considering rectangle {:?} to {:?}",
            point_a_real, point_b_real
        );
        for x in min(point_a.x, point_b.x)..=max(point_a.x, point_b.x) {
            for y in min(point_a.y, point_b.y)..=max(point_a.y, point_b.y) {
                if outside_points.contains(&Point { x: x, y: y }) {
                    println!(
                        "Found outside point at compressed coord {:?},{:?}, {:?},{:?}",
                        x, y, sorted_x[x], sorted_y[y]
                    );
                    contains_outside = true;
                    break;
                }
            }
            if contains_outside {
                break;
            }
        }
        if !contains_outside {
            let area = point_a_real.area(&point_b_real);
            if area > max_area {
                println!(
                    "New max area found: {}, {:?} to {:?}",
                    area, point_a_real, point_b_real
                );
                max_area = area;
            }
        }
    }

    println!("max area: {:?}", max_area);
}
