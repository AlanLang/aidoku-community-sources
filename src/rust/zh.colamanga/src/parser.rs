use aidoku::{
	error::{AidokuError, Result},
	prelude::format,
	std::{html::Node, json, ArrayRef, ObjectRef, String, ValueRef, Vec},
	Manga, MangaPageResult, MangaStatus,
};
use alloc::string::ToString;

pub trait MangaListResponse {
	fn get_page_result(self) -> Result<MangaPageResult>;
}

impl MangaListResponse for Node {
	fn get_page_result(self) -> Result<MangaPageResult> {
		let manga = self.select("li.fed-list-item").array().get_manga_list()?;

		let has_more = false;

		Ok(MangaPageResult { manga, has_more })
	}
}

pub trait Element {
	fn get_attr(&self, selector: &str, attr: &str) -> String;
	fn get_text(&self, selector: &str) -> String;
}

impl Element for Node {
	fn get_attr(&self, selector: &str, attr: &str) -> String {
		self.select(selector).attr(attr).read()
	}

	fn get_text(&self, selector: &str) -> String {
		self.select(selector).text().read()
	}
}
pub trait JsonString {
	fn json(self) -> Result<ValueRef>;
}

impl JsonString for String {
	fn json(self) -> Result<ValueRef> {
		json::parse(self)
	}
}

trait MangaArr {
	fn get_manga_list(self) -> Result<Vec<Manga>>;
}
impl MangaArr for ArrayRef {
	fn get_manga_list(self) -> Result<Vec<Manga>> {
		todo!()
	}
}
