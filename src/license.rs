#[derive(Debug)]
pub struct License {
    name: String,
    spdx_id: Option<String>,
    category: String,
    content: String,
    pub slug: String,
}

impl License {
    pub fn new(
        name: String,
        spdx_id: Option<String>,
        category: String,
        content: String,
        slug: String,
    ) -> License {
        License {
            name,
            spdx_id,
            category,
            content,
            slug,
        }
    }
}
