use itertools::sorted;
use optsort::output::optsort;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();


    const N: usize = 7;

    for _ in 0..1000000 {
        let rand_array: [usize; N] = std::array::from_fn(|_| rng.gen_range(0..100));
        assert_eq!(
            optsort::optsort7(rand_array)
                .into_iter()
                .collect::<Vec<usize>>(),
            sorted(rand_array).into_iter().collect::<Vec<usize>>()
        );
        // println!("{:?}", optsort::optsort6(rand_array));
    }

    // println!("{:?}", optsort::optsort3([32, 11, 9]));
    // println!("{:?}", optsort::optsort4([9, 1, 10, 3]));
    // println!("{:?}", optsort::optsort5([293, 2, 923, 1, 20]));
    // println!("{:?}", optsort::optsort6([1, 5, 2, 5, 3, 9]));
}
