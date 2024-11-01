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
		let mut manga = Vec::<Manga>::new();
		for item in self {
			manga.push(get_mange(item)?);
		}
		Ok(manga)
	}
}

fn get_mange(item: ValueRef) -> Result<Manga> {
	let node = item.as_node()?;

	let cover = node.select("a.fed-list-pics").attr("data-original").read();
	let title = node.select("a.fed-list-title").text().read();
	let id = node.select("a.fed-list-pics").attr("href").read();
	let url = id.clone();

	aidoku::prelude::println!("{} {}", title, url);

	Ok(Manga {
		id,
		cover,
		title,
		url,
		..Default::default()
	})
}
