use self::kyu6::most_frequent_days::most_frequent_days;
use self::kyu6::pascals_triangle::pascals_triangle::pascals_triangle;
use self::kyu6::simpsons_rule_approximation_integration::simpsons_rule_approximation_integration::simpson;
use self::kyu6::versions_manager::VersionManager;
mod kyu6;
use self::kyu5::easy_cyclists_training::easy_cyclists_training::temps;
use self::kyu5::factorial_decomposition::factorial_decomposition::decomp;
use self::kyu5::going_to_zero_or_infinity::going_to_zero_or_infinity::going;
use self::kyu5::human_readable_time::human_readable_time::make_readable;
mod kyu5;
use self::kyu4::human_readable_duration_format::human_readable_duration_format::format_duration;
use self::kyu4::organize_a_round_robin_tournament::organize_a_round_robin_tournament::build_matches_table;
use self::kyu4::simplifying_multilinear_polynomials::simplifying_multilinear_polynomials::simplify;
use self::kyu4::square_into_square_protect_trees::square_into_square_protect_trees::decompose;
use self::kyu4::total_increasing_or_decreasing_numbers_up_to_a_power_of_10::total_increasing_or_decreasing_numbers_up_to_a_power_of_10::total_inc_dec;
mod kyu4;
use self::kyu3::prime_streaming_13::prime_streaming_13::stream;
use self::kyu3::the_lift::the_lift::the_lift;
mod kyu3;

fn main() {
    println!("VersionManager: {:?}", VersionManager::from_version("0.0").unwrap().major().release());

    println!("most_frequent_days: {:?}", most_frequent_days(2024));

    println!("human_readable_time: {}", make_readable(359999));

    for i in 1..50 {
        println!("going_to_zero_or_inifinity: {}", going(i));
    }

    println!("factorial_decomposition: {}", decomp(50));

    println!("easy_cyclists_training: {}", temps(30, 5, 30));

    println!("total_increasing_or_decreasing_numbers_up_to_a_power_of_10: {}", total_inc_dec(4));

    println!("square_into_square_protect_trees: {:?}", decompose(44));

    println!("simplifying_multilinear_polynomials: {}", simplify("3a+b+4ac+bc-ab+3a-cb-a-a"));

    println!("organize_a_round_robin_tournament: {:?}", build_matches_table(4));

    println!("human_readable_duration_format: {}", format_duration(62));

    println!("{:?}", pascals_triangle(4));

    let mut prime_iterator = stream();
    for _ in 0..10 {println!("{:?}", prime_iterator.next().unwrap())}

    println!("{:?}", the_lift(&[vec![], vec![], vec![5,5,5],vec![],vec![],vec![],vec![]], 5));

    println!("{}", simpson(290));
}
