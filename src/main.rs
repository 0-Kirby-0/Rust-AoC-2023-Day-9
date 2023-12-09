use color_eyre::eyre::Result;

/*I have an unhealthy appreciation for member functions.
Below you can see the effort I will go to, so I can call
x.func() instead of func(x) :)*/
trait Sequence {
    fn derive(&self) -> Vec<i32>;
    fn extrapolate_extrema(&self) -> (i32, i32);
}

impl<C> Sequence for C
where
    C: std::ops::Deref<Target = [i32]>,
{
    fn derive(&self) -> Vec<i32> {
        self.windows(2).map(|slc| slc[1] - slc[0]).collect()
    }
    ///Extends the sequence by one value in both directions, and returns both new values.
    ///This should really be two functions, but that would lead to heavy code duplication
    ///as well as lost performance.
    fn extrapolate_extrema(&self) -> (i32, i32) {
        let mut current_derivation: Vec<i32> = self.iter().copied().collect();
        let mut first_in_sequence: Vec<i32> = vec![];
        let mut last_in_sequence: Vec<i32> = vec![];
        loop {
            if current_derivation.iter().all(|i| *i == 0) {
                break;
            }
            first_in_sequence.push(*current_derivation.first().unwrap());
            last_in_sequence.push(*current_derivation.last().unwrap());
            current_derivation = current_derivation.derive();
        }
        println!("{:?}", first_in_sequence);
        (
            first_in_sequence.iter().rev().fold(0, |acc, v| v - acc),
            last_in_sequence.iter().sum(), //luckily, summation is commutative. And has an std impl :)
        )
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = aoc_input_manager::get_input_string();
    let sequences = parse_input(&input);

    let (extr_min_sum, extr_max_sum): (i32, i32) = sequences
        .iter()
        .map(|s| s.extrapolate_extrema())
        .fold((0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1));

    println!("The solution for Part 1 is {}.", extr_max_sum);
    println!("The solution for Part 2 is {}.", extr_min_sum);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().expect("Input data is malformed."))
                .collect()
        })
        .collect()
}
