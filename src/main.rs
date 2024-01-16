use self::kyu6::versions_manager::VersionManager;
use self::kyu6::most_frequent_days::most_frequent_days;
mod kyu6;

fn main() {
    println!("{:?}", VersionManager::from_version("0.0").unwrap().major().release());

    println!("{:?}", most_frequent_days(2024));
}
