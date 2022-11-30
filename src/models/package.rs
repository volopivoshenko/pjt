use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Response {
    info: Package,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    author: String,
    author_email: String,
    bugtrack_url: String,
    classifiers: Vec<String>,
    description: String,
    description_content_type: String,
    docs_url: String,
    download_url: String,
    home_page: String,
    keywords: Vec<String>,
    
}
