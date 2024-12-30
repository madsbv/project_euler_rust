mod id1;
use phf::phf_map;

type SolutionFunction = fn() -> i64;

static SOLUTION_FUNCTIONS: phf::Map<u64, (SolutionFunction, bool)> = phf_map! {
    1u64 => (id1::solution, true),
};

fn main() {
    let problem_id = 1;
    solve_problem(problem_id);
}

fn solve_problem(id: u64) {
    let (solution, verified) = SOLUTION_FUNCTIONS
        .get(&id)
        .expect("Problem {id} not solved, or solution not added to phf_map correctly");

    println!("The answer to problem {id} is: {}", solution());
    if *verified {
        println!("This answer has been verified as correct");
    } else {
        println!("This answer HAS NOT been verified yet");
    }
}
