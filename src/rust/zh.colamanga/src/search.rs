use core::fmt::{Display, Formatter, Result};

use aidoku::helpers::uri::QueryParameters;
use alloc::{
	borrow::ToOwned,
	string::{String, ToString},
};

#[derive(Default)]
pub struct Search {
	page: i32,
	keyword: String,
}

impl Search {
	pub fn new<S: AsRef<str>>(page: i32, keyword: S) -> Self {
		Self {
			page,
			keyword: keyword.as_ref().to_owned(),
			..Default::default()
		}
	}
}

impl Display for Search {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let mut query = QueryParameters::new();
		query.push_encoded("type", Some("1"));
		query.push("searchString", Some(&self.keyword));
		query.push("page", Some(&self.page.to_string()));
		write!(f, "{query}")
	}
}
