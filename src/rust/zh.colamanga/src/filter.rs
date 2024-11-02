use alloc::string::{String, ToString};
use strum_macros::{Display, FromRepr};

#[derive(Default, Display, FromRepr)]
pub enum Status {
	#[default]
	#[strum(to_string = "0")]
	All,

	#[strum(to_string = "1")]
	Ongoing,

	#[strum(to_string = "2")]
	Completed,
}

#[derive(Default, Display)]
pub enum Sort {
	#[default]
	#[strum(to_string = "1")]
	LastUpdated,
}

pub fn get_sort_code(index: usize) -> String {
	let sorts = [
		"none",
		"update",
		"dailyCount",
		"weeklyCount",
		"monthlyCount",
	];
	sorts[index].to_string()
}

pub fn get_kind_code(index: usize) -> String {
	let kinds = [
		"10023", "10024", "10126", "10210", "10143", "10124", "10129", "10242", "10560", "10122",
		"10641", "10309", "10461", "11224", "10943", "10201", "10321", "10138", "10301", "10722",
		"12044", "10125", "12123", "10131", "10133", "10453", "10480", "10706", "10127", "10142",
	];
	kinds[index].to_string()
}

pub fn get_status_code(index: usize) -> String {
	let status = [Status::All, Status::Ongoing, Status::Completed];
	status[index].to_string()
}
