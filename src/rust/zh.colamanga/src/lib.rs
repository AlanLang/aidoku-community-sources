#![no_std]
extern crate alloc;
mod filter;
mod parser;
mod search;
mod url;

use aidoku::{
	error::Result,
	prelude::*,
	std::net::Request,
	std::{String, Vec},
	Chapter, Filter, Manga, MangaPageResult, Page,
};
use parser::MangaListResponse;
use url::Url;
const BASE_URL: &str = "https://www.colamanga.com/";

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let manga_list_url = Url::from((filters, page));
	if let Url::Filters { .. } = manga_list_url {
		let filters_page = manga_list_url.get_html()?;
		return filters_page.get_page_result();
	}
	let filters_page = manga_list_url.get_html()?;
	return filters_page.get_search_result();
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
	let manga_page = Url::Manga { id: &manga_id }.get_html()?;
	return manga_page.get_details_result(&manga_id);
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let manga_page = Url::Manga { id: &manga_id }.get_html()?;
	return manga_page.get_chapter_list_result();
}

#[get_page_list]
fn get_page_list(_: String, _: String) -> Result<Vec<Page>> {
	todo!()
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	request.header("Referer", BASE_URL).header(
		"User-Agent",
		"Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) \
			 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
	);
}
