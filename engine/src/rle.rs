/// Parses RLE (Run Length Encoded) format into a list of alive cell coordinates.
/// Handles comment lines (#), header lines (x = ...) and the standard b/o/$ /! tags.
pub fn parse(input: &str) -> Result<Vec<(i32, i32)>, String> {
    let mut cells = Vec::new();
    let mut x = 0i32;
    let mut y = 0i32;
    let mut run = 0i32;

    for line in input.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.starts_with('x') || line.is_empty() {
            continue;
        }
        for ch in line.chars() {
            match ch {
                '0'..='9' => run = run * 10 + (ch as i32 - '0' as i32),
                'b' | '.' => {
                    x += run.max(1);
                    run = 0;
                }
                'o' => {
                    let n = run.max(1);
                    for i in 0..n {
                        cells.push((x + i, y));
                    }
                    x += n;
                    run = 0;
                }
                '$' => {
                    y += run.max(1);
                    x = 0;
                    run = 0;
                }
                '!' => return Ok(cells),
                _ => {}
            }
        }
    }

    Ok(cells)
}
