#[derive(Debug)]
struct Site {
    x: u32,
    y: u32,
    grain: u64
}

fn main() {
    let site = Site {
        x: 0,
        y: 0,
        grain: 4
    };

    let mut v: Vec<Vec<Site>> = Vec::new();

    for x in 1..3 {
        for y in 1..3 {
            v[x][y] = site;
        }
    }

    println!("Site is {:?}", v);
}
