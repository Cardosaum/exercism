pub fn count(lines: &[&str]) -> u32 {
    dbg!(&lines);
    let mut v: Vec<char> = Vec::new();
    let l = lines[0].len();
    let mut r = 0;
    lines
        .iter()
        .for_each(|line| line.chars().for_each(|c| v.push(c)));
    v.iter().enumerate().for_each(|(i, &c)| {
        if c == '+' {
            r += find_rectangle(i, &v, &l);
        }
    });
    dbg!(v, l, r);
    r as u32
}

fn find_rectangle(i: usize, v: &Vec<char>, length: &usize) -> usize {
    // TODO Check if there is another cross in the right side of the one we just
    // found
    let upper_right_vertices: Vec<(usize, char)> = v
        .iter()
        .enumerate()
        .filter(|(j, &c)| j > &i && c == '+' && j / length == i / length)
        .map(|(j, &c)| (j, c))
        .collect();

    // TODO Check if there is a cross below
    let lower_left_vertices: Vec<(usize, char)> = v
        .iter()
        .enumerate()
        .filter(|(j, &c)| j > &i && c == '+' && j % length == i % length)
        .map(|(j, &c)| (j, c))
        .collect();

    // TODO Check if there is a final cross completing the rectangle
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

    // TODO Check if edges are valid
    let f: Vec<(usize, char)> = lower_right_vertices
        .iter()
        .filter(|(j, _)| {
            !v[i..*j].iter().enumerate().any(|(k, &c)| {
                false
                    || (k > i
                        && k / length == i / length
                        && k % length > i % length
                        && k % length < j % length
                        && (c != '+' && c != '-'))
                    || (k > i
                        && k / length == j / length
                        && k % length > i % length
                        && k % length < j % length
                        && (c != '+' && c != '-'))
                    || (k > i && k % length == i % length && (c != '+' && c != '|'))
                    || (k > i && k % length == j % length && (c != '+' && c != '|'))
            })
        })
        .map(|(j, c)| (*j, *c))
        .collect();

    dbg!(f);

    lower_right_vertices.len()
}
