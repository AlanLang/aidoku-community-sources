use aidoku::{
	error::Result,
	helpers::uri::QueryParameters,
	prelude::format,
	std::{html::Node, net::Request, String, ValueRef, Vec},
	Filter, FilterType,
};
use alloc::{borrow::ToOwned as _, string::ToString};
use core::fmt::{Display, Formatter, Result as FmtResult};
use strum_macros::{Display, FromRepr, IntoStaticStr};

use crate::filter::{get_kind_code, get_sort_code, get_status_code, Status};

const STATUS: [Status; 3] = [Status::All, Status::Ongoing, Status::Completed];

#[expect(private_interfaces)]
#[derive(Display)]
#[strum(prefix = "https://www.colamanga.com")]
pub enum Url<'a> {
	#[strum(to_string = "/")]
	Domain,

	#[strum(to_string = "/")]
	Home,
	// status={status}&mainCategoryId={kind}&orderBy={sort_by}&page={page}
	#[strum(to_string = "/show?{query}")]
	Filters { query: QueryParameters },

	#[strum(to_string = "/comic/{id}")]
	Manga { id: &'a str },
}

impl<'a> Url<'a> {
	pub fn get_html(self) -> Result<Node> {
		self.get().html()
	}
}

impl Url<'_> {
	pub fn get(&self) -> Request {
		aidoku::prelude::println!("url: {}", self.to_string());
		Request::get(self.to_string()).default_headers()
	}
}

impl<'a> From<(Vec<Filter>, i32)> for Url<'a> {
	fn from((filters, page): (Vec<Filter>, i32)) -> Self {
		let mut query = QueryParameters::new();

		for filter in filters {
			match filter.kind {
				FilterType::Select => {
					let name = filter.name;
					let index = filter.value.as_int().unwrap_or(0) as usize;
					if index == 0 {
						continue;
					}
					aidoku::prelude::println!("filter name: {}, index: {}", name, index);

					if name == "分类" {
						let kind_code = get_kind_code(index);
						query.push_encoded("mainCategoryId", Some(&kind_code))
					}
					if name == "连载状态" {
						let status = get_status_code(index);
						query.push_encoded("status", Some(status.to_string().as_str()))
					}
					if name == "排序" {
						let sort = get_sort_code(index);
						query.push_encoded("orderBy", Some(&sort))
					}
				}
				_ => continue,
			}
		}
		query.push_encoded("page", Some(page.to_string().as_str()));
		Url::Filters { query }
	}
}

pub trait DefaultRequest {
	fn default_headers(self) -> Self;
}

impl DefaultRequest for Request {
	fn default_headers(self) -> Self {
		let referer = Url::Domain.to_string();
		self.header("Referer", &referer).header(
			"User-Agent",
			"Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) \
			 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
		)
	}
}
