fn idx2(x: usize, y: usize, pitch: usize) -> usize {
    y * pitch + x
}

fn main() {
    let mut input = String::new();
    for s in std::env::args().skip(1) {
        for c in s.chars() {
            input.push(c.to_uppercase().next().expect("No uppercase variant"));
        }
    }
    let len = input.len();

    let mut output = vec![];

    let width = len * 3;
    let height = len * 3 / 2;
    for _ in 0..height {
        for _ in 0..width {
            output.push(' ');
        }
        output.push('\n');
    }
    output.pop();
    // Compensate for '\n'
    let width = width + 1;

/*
    T E S T
  / E   / S
T E S T   E
E   T S E T
S /   E /
T E S T


        T E S T T E S T
      / E           / S
    /   S         /   E
  /     T       /     T
T E S T T E S T       T
E       E     S       S
S       S     E       E
T       T S E T T S E T
T     /       T     /
E   /         S   /
S /           E /
T S E T T S E T


*/
{
    let mut draw2d = |start_x, start_y| {
        for (i, c) in input.chars().enumerate() {
            // Top row
            output[idx2(i * 2 + start_x, start_y, width)] = c;
            // Left column
            output[idx2(start_x, i + start_y, width)] = c;
            // Bottom row
            output[idx2((len - 1) * 2 - (i * 2) + start_x, start_y + len - 1, width)] = c;
            // Right column
            output[idx2((len - 1) * 2 + start_x ,start_y + len - 1 - i, width)] = c;
        }
    };

    draw2d(if len % 2  == 0{len } else { len - 1}, 0);
    draw2d(0, len / 2);
}

    for i in 1..len/2 {
        output[idx2((len / 2 - i) * 2, i, width)] = '/';
        output[idx2((len / 2 - i) * 2, i + len - 1, width)] = '/';
        output[idx2((len / 2 - i) * 2 + len * 2 - 2, i, width)] = '/';
        output[idx2((len / 2 - i) * 2 + len * 2 - 2, i + len - 1, width)] = '/';
    }



    println!("{}", output.into_iter().collect::<String>());

}
