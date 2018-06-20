const WIDTH: i32 = 40;
const HEIGHT: i32 = 40;
const MAX_GRAINS: i64 = 3;

type Map = Vec<Vec<i64>>;

struct Point(i32, i32);

fn ns(num : i64) -> Map {
    vec![vec![num; HEIGHT as usize]; WIDTH as usize]
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

fn add(one: &Map, two: &Map) -> Map {
    let mut mat = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    for x in 0..WIDTH as usize{
        for y in 0..HEIGHT as usize{
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
    let mut id =  subtract(&ns(6), &mut sixes);
    update_map(&mut id);
    id
}

fn main() {
    print_map(&identity());
}
