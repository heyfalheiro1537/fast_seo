pub mod analyzer;
pub mod meta;
pub mod sitemap;
pub mod keywords;
pub mod performance;
pub mod structured_data;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoReport {
    pub url: String,
    pub title: Option<String>,
    pub meta_description: Option<String>,
    pub h1_tags: Vec<String>,
    pub h2_tags: Vec<String>,
    pub keyword_density: HashMap<String, f32>,
    pub images_without_alt: u32,
    pub internal_links: u32,
    pub external_links: u32,
    pub page_size: Option<u64>,
    pub load_time: Option<f64>,
    pub structured_data: Vec<String>,
    pub issues: Vec<SeoIssue>,
    pub score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoIssue {
    pub severity: IssueSeverity,
    pub message: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Critical,
    Warning,
    Info,
}