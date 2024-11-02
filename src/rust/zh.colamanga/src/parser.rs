use crate::url::Url;
use aidoku::{
	error::Result,
	prelude::*,
	std::{html::Node, ArrayRef, ValueRef, Vec},
	Chapter, Manga, MangaPageResult, MangaStatus,
};
use alloc::{
	fmt::format,
	string::{String, ToString},
};

pub trait MangaListResponse {
	fn get_page_result(self) -> Result<MangaPageResult>;
	fn get_search_result(self) -> Result<MangaPageResult>;
	fn get_details_result(self, mange_id: &str) -> Result<Manga>;
	fn get_chapter_list_result(self) -> Result<Vec<Chapter>>;
	fn get_content(&self, name: &str) -> Node;
}

impl MangaListResponse for Node {
	fn get_page_result(self) -> Result<MangaPageResult> {
		let manga = self
			.select("li.fed-list-item")
			.array()
			.get_manga_list(get_home_page_mange)?;
		let page = self.select("#fed-now").text().read().parse::<usize>();

		let has_more = match page {
			// can't fine the total page in web, just take a fixed amount of data
			Ok(index) => index < 26,
			Err(_) => false,
		};

		Ok(MangaPageResult { manga, has_more })
	}

	fn get_content(&self, name: &str) -> Node {
		let selector = format!(
			".fed-part-rows .fed-col-xs12:has(.fed-text-muted:containsOwn({})) > a",
			name,
		);
		self.select(".fed-deta-content").select(selector)
	}

	fn get_search_result(self) -> Result<MangaPageResult> {
		let manga = self
			.select("dl.fed-deta-info")
			.array()
			.get_manga_list(get_search_page_mange)?;
		let has_more = self
			.select(".fed-btns-disad:contains(下页)")
			.array()
			.is_empty();
		aidoku::prelude::println!("hasMore {}", has_more);
		Ok(MangaPageResult {
			manga,
			has_more: has_more,
		})
	}

	fn get_details_result(self, manga_id: &str) -> Result<Manga> {
		let cover = self.select("a.fed-list-pics").attr("data-original").read();
		let title = self
			.select(".fed-deta-content")
			.select("fed-part-eone")
			.text()
			.read();
		let description = self
			.select(".fed-deta-content")
			.select(".fed-part-esan")
			.text()
			.read();
		let manga_url = Url::Manga { id: &manga_id }.to_string();
		let author = self.get_content("作者").text().read();
		let status_str = self.get_content("状态").text().read();
		aidoku::prelude::println!("author {}", author);
		aidoku::prelude::println!("status_str {}", status_str);
		let status = match status_str.as_str() {
			"连载中" => MangaStatus::Ongoing,
			"已完结" | "短篇" => MangaStatus::Completed,
			_ => MangaStatus::Unknown,
		};
		let categories = self
			.get_content("类别")
			.array()
			.filter_map(NodeArrValue::ok_text)
			.collect::<Vec<_>>();
		Ok(Manga {
			id: manga_id.to_string(),
			cover,
			title,
			artist: author.clone(),
			author,
			description,
			url: manga_url,
			categories,
			status,
			..Default::default()
		})
	}

	fn get_chapter_list_result(self) -> Result<Vec<Chapter>> {
		let chapters = self
			.select(".fed-play-item")
			.select("a.fed-rims-info")
			.array()
			.filter_map(get_some_chapter)
			.collect::<Vec<_>>();
		Ok(chapters)
	}
}

trait MangaArr {
	fn get_manga_list<F>(self, parser: F) -> Result<Vec<Manga>>
	where
		F: Fn(ValueRef) -> Result<Manga>;
}
impl MangaArr for ArrayRef {
	fn get_manga_list<F>(self, parser: F) -> Result<Vec<Manga>>
	where
		F: Fn(ValueRef) -> Result<Manga>,
	{
		let mut manga = Vec::<Manga>::new();
		for item in self {
			manga.push(parser(item)?);
		}
		Ok(manga)
	}
}

fn get_home_page_mange(item: ValueRef) -> Result<Manga> {
	let node = item.as_node()?;

	let cover = node.select("a.fed-list-pics").attr("data-original").read();
	let title = node.select("a.fed-list-title").text().read();
	let id = node.select("a.fed-list-pics").attr("href").read();
	let url = Url::Manga { id: &id }.to_string();

	Ok(Manga {
		id,
		cover,
		title,
		url,
		..Default::default()
	})
}

fn get_search_page_mange(item: ValueRef) -> Result<Manga> {
	let node = item.as_node()?;

	let cover = node.select("a.fed-list-pics").attr("data-original").read();
	let title = node
		.select("dd.fed-deta-content")
		.select(".fed-part-eone")
		.text()
		.read();
	let id = node
		.select("dd.fed-deta-content")
		.select(".fed-part-eone")
		.select("a")
		.attr("href")
		.read();
	let url = Url::Manga { id: &id }.to_string();

	Ok(Manga {
		id,
		cover,
		title,
		url,
		..Default::default()
	})
}

pub trait NodeArrValue {
	fn ok_text(self) -> Option<String>;
}

impl NodeArrValue for ValueRef {
	fn ok_text(self) -> Option<String> {
		self.as_node().map(|node| node.text().read()).ok()
	}
}

fn get_some_chapter(node: ValueRef) -> Option<Chapter> {
	get_chapter(node).ok()
}

fn get_chapter(item: ValueRef) -> Result<Chapter> {
	let node = item.as_node()?;

	let id = node.attr("href").read();
	let title = node.text().read();
	let date_updated: f64 = -1.0;
	let volume: f32 = -1.0;
	let scanlator = "".to_string();
	let url = Url::Manga { id: &id }.to_string();
	Ok(Chapter {
		id,
		title,
		date_updated,
		volume,
		chapter: -1.0,
		scanlator,
		url,
		lang: "zh".to_string(),
	})
}
