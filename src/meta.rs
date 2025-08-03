use scraper::{Html, Selector};
use std::collections::HashMap;

pub struct MetaAnalyzer;

impl MetaAnalyzer {
    pub fn extract_all_meta_tags(document: &Html) -> HashMap<String, String> {
        let mut meta_tags = HashMap::new();
        
        let selector = Selector::parse("meta").unwrap();
        for element in document.select(&selector) {
            let attrs = element.value();
            
            if let Some(name) = attrs.attr("name") {
                if let Some(content) = attrs.attr("content") {
                    meta_tags.insert(name.to_string(), content.to_string());
                }
            }
            
            if let Some(property) = attrs.attr("property") {
                if let Some(content) = attrs.attr("content") {
                    meta_tags.insert(property.to_string(), content.to_string());
                }
            }
        }
        
        meta_tags
    }

    pub fn check_open_graph(document: &Html) -> Vec<String> {
        let mut og_tags = Vec::new();
        let required_og_tags = ["og:title", "og:description", "og:image", "og:url"];
        
        let meta_tags = Self::extract_all_meta_tags(document);
        
        for required_tag in &required_og_tags {
            if meta_tags.contains_key(*required_tag) {
                og_tags.push(required_tag.to_string());
            }
        }
        
        og_tags
    }
}