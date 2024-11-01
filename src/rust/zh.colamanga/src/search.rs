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
