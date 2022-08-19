#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical(usize),
}

pub fn count(lines: &[&str]) -> u32 {
    let mut v: Vec<char> = Vec::new();
    if let Some(l) = lines.get(0) {
        let mut r = 0;
        lines
            .iter()
            .for_each(|line| line.chars().for_each(|c| v.push(c)));
        v.iter().enumerate().for_each(|(i, &c)| {
            if c == '+' {
                r += find_rectangle(i, &v, &l.len());
            }
        });
        r as u32
    } else {
        0
    }
}

fn find_rectangle(i: usize, v: &Vec<char>, length: &usize) -> usize {
    // Check if there is another cross in the right side of the one we just
    // found
    let upper_right_vertices: Vec<(usize, char)> = v
        .iter()
        .enumerate()
        .filter(|(j, &c)| j > &i && c == '+' && j / length == i / length)
        .map(|(j, &c)| (j, c))
        .collect();

    // Check if there is a cross below
    let lower_left_vertices: Vec<(usize, char)> = v
        .iter()
        .enumerate()
        .filter(|(j, &c)| j > &i && c == '+' && j % length == i % length)
        .map(|(j, &c)| (j, c))
        .collect();

    // Check if there is a final cross completing the rectangle
    // Return a point of each edge
    let lower_right_vertices: Vec<(usize, char)> = v
        .iter()
        .enumerate()
        .filter(|(j, &c)| {
            j > &i
                && c == '+'
                && upper_right_vertices
                    .iter()
                    .any(|(k, _)| j % length == k % length && j > k)
                && lower_left_vertices
                    .iter()
                    .any(|(l, _)| j / length == l / length && j > l)
        })
        .map(|(j, &c)| (j, c))
        .collect();

    // Check if edges are valid
    let f: Vec<(usize, usize)> = lower_right_vertices
        .iter()
        .filter(|(j, _)| {
            is_valid_line(i, i + j % length - i % length, Direction::Horizontal, &v)
                && is_valid_line(
                    i,
                    j - (j % length - i % length),
                    Direction::Vertical(*length),
                    &v,
                )
                && is_valid_line(j - (j % length - i % length), *j, Direction::Horizontal, &v)
                && is_valid_line(
                    i + j % length - i % length,
                    *j,
                    Direction::Vertical(*length),
                    &v,
                )
        })
        .map(|(j, _)| (i, *j))
        .collect();

    f.len()
}

fn is_valid_line(x: usize, y: usize, direction: Direction, v: &Vec<char>) -> bool {
    match direction {
        Direction::Horizontal => !v
            .iter()
            .enumerate()
            .any(|(i, &c)| i >= x && i <= y && (c != '+' && c != '-')),
        Direction::Vertical(l) => !v
            .iter()
            .enumerate()
            .any(|(i, &c)| i >= x && i <= y && i % l == x % l && (c != '+' && c != '|')),
    }
}
