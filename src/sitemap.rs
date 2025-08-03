use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapUrl {
    pub loc: String,
    pub lastmod: Option<String>,
    pub changefreq: Option<String>,
    pub priority: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sitemap {
    pub urls: Vec<SitemapUrl>,
}

pub struct SitemapGenerator;

impl SitemapGenerator {
    pub fn generate_xml(urls: &[SitemapUrl]) -> String {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
        
        for url in urls {
            xml.push_str("  <url>\n");
            xml.push_str(&format!("    <loc>{}</loc>\n", url.loc));
            
            if let Some(ref lastmod) = url.lastmod {
                xml.push_str(&format!("    <lastmod>{}</lastmod>\n", lastmod));
            }
            
            if let Some(ref changefreq) = url.changefreq {
                xml.push_str(&format!("    <changefreq>{}</changefreq>\n", changefreq));
            }
            
            if let Some(priority) = url.priority {
                xml.push_str(&format!("    <priority>{:.1}</priority>\n", priority));
            }
            
            xml.push_str("  </url>\n");
        }
        
        xml.push_str("</urlset>");
        xml
    }

    pub async fn fetch_sitemap(url: &str) -> Result<Sitemap, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        let content = response.text().await?;
        
        // Aqui você implementaria o parsing do XML do sitemap
        // Por simplicidade, retornando um sitemap vazio
        Ok(Sitemap { urls: Vec::new() })
    }
}
