fn main() {
    let data = std::fs::read_to_string("data/02.txt").expect("Could not load data file");

    let wrapping_paper: u32 = data
        .lines()
        .map(|line| {
            let mut dimensions = line.split('x');
            let l = dimensions
                .next()
                .expect("Could not find length")
                .parse::<u32>()
                .expect("Could not parse dimension as integer");
            let w = dimensions
                .next()
                .expect("Could not find width")
                .parse::<u32>()
                .expect("Could not parse dimension as integer");
            let h = dimensions
                .next()
                .expect("Could not find height")
                .parse::<u32>()
                .expect("Could not parse dimension as integer");

            let side1 = l * w;
            let side2 = w * h;
            let side3 = h * l;
            let min_side = u32::min(u32::min(side1, side2), side3);

            2 * (side1 + side2 + side3) + min_side
        })
        .sum();

    println!("Part 1: {}", wrapping_paper);

    let ribbon: u32 = data
        .lines()
        .map(|line| {
            let mut dimensions = line.split('x');
            let l = dimensions
                .next()
                .expect("Could not find length")
                .parse::<u32>()
                .expect("Could not parse dimension as integer");
            let w = dimensions
                .next()
                .expect("Could not find width")
                .parse::<u32>()
                .expect("Could not parse dimension as integer");
            let h = dimensions
                .next()
                .expect("Could not find height")
                .parse::<u32>()
                .expect("Could not parse dimension as integer");

            let perimeter1 = 2 * (l + w);
            let perimeter2 = 2 * (w + h);
            let perimeter3 = 2 * (h + l);
            let min_perimeter = u32::min(u32::min(perimeter1, perimeter2), perimeter3);

            let volume = l * w * h;

            min_perimeter + volume
        })
        .sum();

    println!("Part 2: {}", ribbon);
}
