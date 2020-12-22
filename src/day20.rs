use array2d::Array2D;
use regex::Regex;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    pixels: Array2D<char>,
}

fn copy_into(p: &mut Array2D<char>, c: &Array2D<char>, pos: (usize, usize)) {
    for i in 1..c.num_rows() - 1 {
        for j in 1..c.num_columns() - 1 {
            p.set(pos.0 + i - 1, pos.1 + j - 1, *c.get(i, j).unwrap())
                .expect("OOB");
        }
    }
}

fn find_orientation<F>(p: &Array2D<char>, f: F) -> Array2D<char>
where
    F: Fn(&Array2D<char>) -> bool,
{
    for i in 0..4 {
        let r = rotate(&p, i);
        if f(&r) {
            return r;
        }
    }
    for i in 0..4 {
        let r = flip(&rotate(&p, i));
        if f(&r) {
            return r;
        }
    }
    panic!();
}

fn flip(pixels: &Array2D<char>) -> Array2D<char> {
    let mut rows = pixels.as_rows();
    for r in &mut rows {
        r.reverse();
    }
    Array2D::from_rows(&rows)
}

fn rotate(pixels: &Array2D<char>, b: usize) -> Array2D<char> {
    let mut rot = pixels.clone();
    for _ in 0..b {
        let mut rows = rot.as_rows();
        for r in &mut rows {
            r.reverse();
        }
        rot = Array2D::from_columns(&rows);
    }
    rot
}

fn border(pixels: &Array2D<char>, b: usize) -> Vec<char> {
    match b {
        0 => pixels.as_rows().first().unwrap().clone(),
        1 => pixels.as_columns().last().unwrap().clone(),
        2 => pixels.as_rows().last().unwrap().clone(),
        3 => pixels.as_columns().first().unwrap().clone(),
        _ => panic!(),
    }
}

fn flip_border(pixels: &Array2D<char>, b: usize) -> Vec<char> {
    let mut r = border(pixels, b);
    r.reverse();
    r
}

#[allow(dead_code)]
fn print_image(pixels: &Array2D<char>) {
    for r in pixels.rows_iter() {
        for c in r {
            print!("{}", c);
        }
        print!("\n");
    }
}

fn parse_tile(input: &str) -> Tile {
    let mut lines = input.lines();
    Tile {
        id: scan_fmt!(lines.next().unwrap(), "Tile {d}:", usize).unwrap(),
        pixels: Array2D::from_rows(
            &lines
                .map(|l| l.chars().collect())
                .collect::<Vec<Vec<char>>>(),
        ),
    }
}

#[aoc_generator(day20)]
fn gen(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(parse_tile).collect()
}

fn build_pairings(tiles: &Vec<Tile>) -> HashMap<Vec<char>, Vec<&Tile>> {
    let mut pairings = HashMap::<Vec<char>, Vec<&Tile>>::new();
    for t in tiles {
        for i in 0..4 {
            let b = border(&t.pixels, i);
            let r = flip_border(&t.pixels, i);
            if r != b {
                pairings.entry(r).or_default().push(&t);
            }
            pairings.entry(b).or_default().push(&t);
        }
    }
    pairings
}

fn find_corners<'t>(
    tiles: &'t Vec<Tile>,
    pairings: &HashMap<Vec<char>, Vec<&Tile>>,
) -> Vec<&'t Tile> {
    let mut corners = vec![];
    for t in tiles {
        let mut unmatched = 0;
        for i in 0..4 {
            let b = border(&t.pixels, i);
            let r = flip_border(&t.pixels, i);
            let c = std::cmp::max(
                pairings.get(&b).unwrap().len(),
                pairings.get(&r).unwrap().len(),
            );
            match c {
                1 => {
                    unmatched += 1;
                }
                2 => {
                    // matched
                }
                _ => panic!(), // our cheating caught up with us
            }
        }
        if unmatched == 2 {
            corners.push(t);
        }
    }
    assert!(corners.len() == 4);
    corners
}

fn find_paired(t: &Tile, side: usize, pairings: &HashMap<Vec<char>, Vec<&Tile>>) -> Tile {
    let b = border(&t.pixels, side);
    let mut opts = vec![];
    opts.append(&mut pairings.get(&b).unwrap().clone());
    opts.append(&mut pairings.get(&flip_border(&t.pixels, side)).unwrap().clone());
    for p in opts {
        if p.id != t.id {
            return Tile {
                id: p.id,
                pixels: find_orientation(&p.pixels, |r| {
                    return b == border(&r, (side + 2) % 4);
                }),
            };
        }
    }
    panic!();
}

#[aoc(day20, part1)]
fn part1(tiles: &Vec<Tile>) -> usize {
    let pairings = build_pairings(tiles);
    let corners = find_corners(tiles, &pairings);
    corners.iter().map(|t| t.id).product()
}

fn get(grid: &Array2D<Option<Tile>>, i: usize, j: usize) -> &Tile {
    &grid.get(i, j).unwrap().as_ref().unwrap()
}

#[aoc(day20, part2)]
fn part2(tiles: &Vec<Tile>) -> usize {
    let pairings = build_pairings(tiles);
    let corners = find_corners(tiles, &pairings);

    let tile_size = (tiles.len() as f64).sqrt() as usize;
    let mut grid: Array2D<Option<Tile>> = Array2D::filled_with(None, tile_size, tile_size);

    for i in 0..tile_size {
        for j in 0..tile_size {
            grid.set(
                i,
                j,
                Some(match (i, j) {
                    (0, 0) => Tile {
                        id: corners[0].id,
                        pixels: find_orientation(&corners[0].pixels, |p| {
                            let r = border(&p, 1);
                            let b = border(&p, 2);
                            pairings.get(&r).unwrap().len() == 2
                                && pairings.get(&b).unwrap().len() == 2
                        }),
                    },
                    (0, _) => find_paired(get(&grid, i, j - 1), 2, &pairings),
                    (_, _) => find_paired(get(&grid, i - 1, j), 1, &pairings),
                }),
            )
            .expect("OOB");
        }
    }

    let pixel_size = tiles[0].pixels.num_rows() - 2;
    let mut image: Array2D<char> =
        Array2D::filled_with(' ', tile_size * pixel_size, tile_size * pixel_size);
    for i in 0..tile_size {
        for j in 0..tile_size {
            copy_into(
                &mut image,
                &get(&grid, j, i).pixels,
                (i * pixel_size, j * pixel_size),
            );
        }
    }

    let oriented = find_orientation(&image, |p| {
        count_monsters(&p)> 0
    });
    count_waves(&oriented) - 15 * count_monsters(&oriented)
}

fn count_waves(image: &Array2D<char>) -> usize {
    image.elements_row_major_iter()
        .filter(|c| **c == '#')
        .count()
}

fn count_monsters(image: &Array2D<char>) -> usize {
    let simage: String = image.elements_row_major_iter().collect();

    // In order to make a single regex for this we want to pad out the line wraps
    // with `.` for match anything.
    let line_over_run = (simage.len() as f64).sqrt() as usize - 20;
    let monster = vec![
        "..................#.",
        "#....##....##....###",
        ".#..#..#..#..#..#...",
    ]
    .join(String::from_utf8(vec![b'.'; line_over_run]).expect("OMG").as_str());
    let re_monster = Regex::new(&monster).unwrap();

    let mut count = 0;
    let mut j = 0;
    loop {
        if let Some(m) = re_monster.find_at(&simage, j) {
            count += 1;
            j = m.start() + 1;
        } else {
            break;
        }
    }
    count
}

#[test]
fn test_find() {
    let image = "
.####...#####..#...###..
#####..#..#.#.####..#.#.
.#.#...#.###...#.##.##..
#.#.##.###.#.##.##.#####
..##.###.####..#.####.##
...#.#..##.##...#..#..##
#.##.#..#.#..#..##.#.#..
.###.##.....#...###.#...
#.####.#.#....##.#..#.#.
##...#..#....#..#...####
..#.##...###..#.#####..#
....#.##.#.#####....#...
..##.##.###.....#.##..#.
#...#...###..####....##.
.#.##...#.##.#.#.###...#
#.###.#..####...##..#...
#.###...#.##...#.######.
.###.###.#######..#####.
..##.#..#..#.#######.###
#.#..##.########..#..##.
#.#####..#.#...##..#....
#....##..#.#########..##
#...#.....#..##...###.##
#..###....##.#...##.##.#
    "
    .trim()
    .lines()
    .map(|l| l.to_string())
    .collect::<Vec<_>>();

    assert_eq!(count_monsters(&image), 2);
    assert_eq!(count_chop(&image) - 2 * 15, 273);
}
