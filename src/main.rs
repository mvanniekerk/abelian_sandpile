const WIDTH: i32 = 3;
const HEIGHT: i32 = 3;
const MAX_GRAINS: u64 = 3;

type Map = Vec<Vec<u64>>;

struct Point(i32, i32);


fn make_map() -> Map {
    let map = vec![
        vec![3, 3, 3],
        vec![3, 4, 3],
        vec![3, 3, 3]
    ];
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

    map[x as usize][y as usize] = 0;
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

fn main() {
    let mut map = make_map();

    print_map(&map);

    update_map(&mut map);
    print_map(&map);
}
