use aidoku::{
	error::Result,
	helpers::uri::QueryParameters,
	prelude::format,
	std::{html::Node, net::Request, String, ValueRef, Vec},
	Filter, FilterType,
};
use alloc::{borrow::ToOwned as _, string::ToString};
use core::fmt::{Display, Formatter, Result as FmtResult};
use strum_macros::{Display, IntoStaticStr};

#[expect(private_interfaces)]
#[derive(Display)]
#[strum(prefix = "https://www.colamanga.com")]
pub enum Url<'a> {
	#[strum(to_string = "")]
	Domain,

	#[strum(to_string = "/")]
	Home,

	#[strum(to_string = "/comic/{id}")]
	Manga { id: &'a str },
}

#[expect(dead_code)]
#[derive(Default, IntoStaticStr, Clone, Copy)]
enum SearchType {
	#[default]
	#[strum(to_string = "")]
	All,

	#[strum(to_string = "name")]
	Title,

	#[strum(to_string = "author")]
	Author,

	#[strum(to_string = "local")]
	Translator,
}

#[derive(Default)]
struct Search {
	page: i32,
	keyword: String,
	by: SearchType,
}

impl Search {
	fn new<S: AsRef<str>>(page: i32, keyword: S) -> Self {
		Self {
			page,
			keyword: keyword.as_ref().to_owned(),
			..Default::default()
		}
	}
}

impl Display for Search {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		todo!()
	}
}

impl<'a> Url<'a> {
	pub fn get_html(self) -> Result<Node> {
		self.get().html()
	}

	pub fn get_json(self) -> Result<ValueRef> {
		self.get().json()
	}
}

impl Url<'_> {
	pub fn get(&self) -> Request {
		Request::get(self.to_string()).default_headers()
	}
}

impl<'a> From<(Vec<Filter>, i32)> for Url<'a> {
	fn from((filters, page): (Vec<Filter>, i32)) -> Self {
		Url::Home
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
