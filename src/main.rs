extern crate image;

use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::fs::create_dir_all;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const MAX_GRAINS: i32 = 3;

type Map = Vec<Vec<i32>>;

struct Point(i32, i32);

fn ns(num: i32) -> Map {
    vec![vec![num; HEIGHT as usize]; WIDTH as usize]
}

fn colormap() -> HashMap<i32, [u8; 3], RandomState> {
    let mut map = HashMap::new();

    map.insert(2, [255, 150, 108]);
    map.insert(1, [108, 213, 255]);
    map.insert(0, [201, 108, 255]);
    map.insert(3, [255, 228, 108]);

    map
}

fn update_grain(map: &mut Map, point: Point) {
    let Point(x, y) = point;
    let tile = map[x as usize][y as usize];

    if tile <= MAX_GRAINS {
        return;
    }

    let lower = within_bounds(x, y - 1);
    let left = within_bounds(x - 1, y);
    let right = within_bounds(x + 1, y);
    let upper = within_bounds(x, y + 1);

    map[x as usize][y as usize] -= 4;
    inc_tile(map, lower);
    inc_tile(map, left);
    inc_tile(map, right);
    inc_tile(map, upper);
}

fn inc_tile(map: &mut Map, m_point: Option<Point>) {
    match m_point {
        None => return,
        Some(point) => map[point.0 as usize][point.1 as usize] += 1,
    }
}

fn within_bounds(x: i32, y: i32) -> Option<Point> {
    if x >= WIDTH || y >= HEIGHT || x < 0 || y < 0 {
        return None;
    }

    Some(Point(x, y))
}

fn print_map(map: &Map) {
    for row in map {
        for tile in row {
            print!("{}", tile);
        }
        println!();
    }
    println!();
}

fn in_temp_state(map: &Map) -> bool {
    for row in map {
        for tile in row {
            if tile > &MAX_GRAINS {
                return true;
            }
        }
    }
    false
}

fn update_one_cycle(map: &mut Map) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            update_grain(map, Point(x, y));
        }
    }
}

fn update_map(map: &mut Map) {
    while in_temp_state(map) {
        update_one_cycle(map);
    }
}

fn drop_in_middle(map: &mut Map, grains: i32) {
    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;

    map[mid_x as usize][mid_y as usize] += grains;
}

fn add(one: &Map, two: &Map) -> Map {
    let mut mat = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    for x in 0..WIDTH as usize {
        for y in 0..HEIGHT as usize {
            mat[x][y] = one[x][y] + two[x][y];
        }
    }
    mat
}

fn negate(mat: &Map) -> Map {
    let mut new = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    for x in 0..WIDTH as usize {
        for y in 0..WIDTH as usize {
            new[x][y] = -mat[x][y];
        }
    }
    new
}

fn subtract(one: &Map, two: &Map) -> Map {
    add(&one, &negate(two))
}

fn identity() -> Map {
    let mut sixes = ns(6);
    update_map(&mut sixes);
    let mut id = subtract(&ns(6), &mut sixes);
    update_map(&mut id);
    id
}

fn image(mat: &Map, colormap: &HashMap<i32, [u8; 3], RandomState>, name: String) {
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let color = colormap.get(&mat[x as usize][y as usize]);
        match color {
            None => return,
            Some(color) => *pixel = image::Rgb(*color),
        }
    }
    imgbuf.save(name).unwrap();
}

fn drop_multiple(grains: i32, num_images: i32, directory: &str) {
    let mut mat = ns(0);
    let grains_per_image = grains / num_images;
    let color_map = colormap();

    create_dir_all(directory);
    let name = format!("{}/{:03}.png", directory, 0);
    image(&mat, &color_map, name);

    for current_image in 1..(num_images + 1) {
        let name = format!("{}/{:03}.png", directory, current_image);
        drop_in_middle(&mut mat, grains_per_image);
        update_map(&mut mat);
        image(&mat, &color_map, name);
    }
}

fn main() {
    drop_multiple(i32::pow(2, 10), 2, "series");
}
