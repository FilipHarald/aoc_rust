pub mod p_2015_day_1;
pub mod p_2015_day_2;
pub mod p_2015_day_3;
pub mod p_2015_day_4;
pub mod p_2015_day_5;
pub mod p_2015_day_6;
pub mod p_2015_day_7;

pub mod p_2021_day_12;
pub mod p_2021_day_13;

pub mod p_2022_day_1;
pub mod p_2022_day_2;
pub mod p_2022_day_3;
pub mod p_2022_day_4;
pub mod p_2022_day_5;
pub mod p_2022_day_6;
pub mod p_2022_day_7;
pub mod p_2022_day_8;
pub mod p_2022_day_9;
pub mod p_2022_day_10;
pub mod p_2022_day_11;
pub mod p_2022_day_12;

// TODO: some kind of hash map?
//const PS: HashMap<&str, fn(&str) -> i32> = HashMap::from([
//                         ("20151a", p_2015_day_1::solver::solve_a as fn(&str) -> i32)
//]);
//
//pub fn solve(year: &str, day: &str, part: &str, input: &str) -> i32 {
//    let puzzle_id: String = year.to_owned() + day + part;
//    let fun = PS.get(puzzle_id.as_str()).unwrap();
//    return fun(input);
//}

pub fn solve(year: &str, day: &str, part: &str, input: &str) -> String {
    let id = year.to_owned() + day + part;
    let result = match id.as_str() {
        "20151a"=>p_2015_day_1::solver::solve_a(input),
        "20151b"=>p_2015_day_1::solver::solve_b(input),
        "20152a"=>p_2015_day_2::solver::solve_a(input),
        "20152b"=>p_2015_day_2::solver::solve_b(input),
        "20153a"=>p_2015_day_3::solver::solve_a(input),
        "20153b"=>p_2015_day_3::solver::solve_b(input),
        "20154a"=>p_2015_day_4::solver::solve_a(input),
        "20154b"=>p_2015_day_4::solver::solve_b(input),
        "20155a"=>p_2015_day_5::solver::solve_a(input),
        "20155b"=>p_2015_day_5::solver::solve_b(input),
        "20156a"=>p_2015_day_6::solver::solve_a(input),
        "20156b"=>p_2015_day_6::solver::solve_b(input),
        "20157a"=>p_2015_day_7::solver::solve_a(input),
        "20157b"=>p_2015_day_7::solver::solve_b(input),

        "202112a"=>p_2021_day_12::solver::solve_a(input),
        "202112b"=>p_2021_day_12::solver::solve_b(input),
        "202113a"=>p_2021_day_13::solver::solve_a(input),
        "202113b"=>p_2021_day_13::solver::solve_b(input),

        "20221a"=>p_2022_day_1::solver::solve_a(input),
        "20221b"=>p_2022_day_1::solver::solve_b(input),
        "20222a"=>p_2022_day_2::solver::solve_a(input),
        "20222b"=>p_2022_day_2::solver::solve_b(input),
        "20223a"=>p_2022_day_3::solver::solve_a(input),
        "20223b"=>p_2022_day_3::solver::solve_b(input),
        "20224a"=>p_2022_day_4::a::solve(input),
        "20224b"=>p_2022_day_4::b::solve(input),
        "20225a"=>p_2022_day_5::solver::solve_a(input),
        "20225b"=>p_2022_day_5::solver::solve_b(input),
        "20226a"=>p_2022_day_6::a::solve(input),
        "20226b"=>p_2022_day_6::b::solve(input),
        "20227a"=>p_2022_day_7::a::solve(input),
        "20227b"=>p_2022_day_7::b::solve(input),
        "20228a"=>p_2022_day_8::a::solve(input),
        "20228b"=>p_2022_day_8::b::solve(input),
        "20229a"=>p_2022_day_9::a::solve(input),
        "20229b"=>p_2022_day_9::b::solve(input),
        "202210a"=>p_2022_day_10::a::solve(input),
        "202210b"=>p_2022_day_10::b::solve(input),
        "202211a"=>p_2022_day_11::a::solve(input),
        "202211b"=>p_2022_day_11::b::solve(input),
        "202212a"=>p_2022_day_12::a::solve(input),
        "202212b"=>p_2022_day_12::b::solve(input),
        _=>todo!(),
    };
    return result;
}
