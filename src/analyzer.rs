use crate::{SeoReport, SeoIssue, IssueSeverity};
use scraper::{Html, Selector};
use std::collections::HashMap;
use url::Url;

pub struct SeoAnalyzer {
    client: reqwest::Client,
}

impl SeoAnalyzer {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn analyze_url(&self, url: &str) -> Result<SeoReport, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        
        let response = self.client.get(url).send().await?;
        let html_content = response.text().await?;
        let load_time = start_time.elapsed().as_secs_f64();
        
        let document = Html::parse_document(&html_content);
        
        let mut report = SeoReport {
            url: url.to_string(),
            title: self.extract_title(&document),
            meta_description: self.extract_meta_description(&document),
            h1_tags: self.extract_h1_tags(&document),
            h2_tags: self.extract_h2_tags(&document),
            keyword_density: self.calculate_keyword_density(&html_content),
            images_without_alt: self.count_images_without_alt(&document),
            internal_links: 0,
            external_links: 0,
            page_size: Some(html_content.len() as u64),
            load_time: Some(load_time),
            structured_data: self.extract_structured_data(&document),
            issues: Vec::new(),
            score: 0,
        };

        let (internal, external) = self.count_links(&document, url)?;
        report.internal_links = internal;
        report.external_links = external;

        report.issues = self.generate_issues(&report);
        report.score = self.calculate_score(&report);

        Ok(report)
    }

    fn extract_title(&self, document: &Html) -> Option<String> {
        let selector = Selector::parse("title").unwrap();
        document
            .select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
    }

    fn extract_meta_description(&self, document: &Html) -> Option<String> {
        let selector = Selector::parse("meta[name='description']").unwrap();
        document
            .select(&selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(|s| s.to_string())
    }

    fn extract_h1_tags(&self, document: &Html) -> Vec<String> {
        let selector = Selector::parse("h1").unwrap();
        document
            .select(&selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .collect()
    }

    fn extract_h2_tags(&self, document: &Html) -> Vec<String> {
        let selector = Selector::parse("h2").unwrap();
        document
            .select(&selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .collect()
    }

    fn calculate_keyword_density(&self, html_content: &str) -> HashMap<String, f32> {
        let text = html_content
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();
        
        let words: Vec<&str> = text
            .split_whitespace()
            .filter(|word| word.len() > 3)
            .collect();

        let total_words = words.len() as f32;
        let mut word_count = HashMap::new();

        for word in words {
            let word = word.to_lowercase();
            *word_count.entry(word).or_insert(0) += 1;
        }

        word_count
            .into_iter()
            .map(|(word, count)| (word, (count as f32 / total_words) * 100.0))
            .collect()
    }

    fn count_images_without_alt(&self, document: &Html) -> u32 {
        let selector = Selector::parse("img").unwrap();
        document
            .select(&selector)
            .filter(|el| el.value().attr("alt").is_none() || el.value().attr("alt").unwrap().is_empty())
            .count() as u32
    }

    fn count_links(&self, document: &Html, base_url: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
        let selector = Selector::parse("a[href]").unwrap();
        let base = Url::parse(base_url)?;
        
        let mut internal = 0;
        let mut external = 0;

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(url) = base.join(href) {
                    if url.host() == base.host() {
                        internal += 1;
                    } else {
                        external += 1;
                    }
                }
            }
        }

        Ok((internal, external))
    }

    fn extract_structured_data(&self, document: &Html) -> Vec<String> {
        let selector = Selector::parse("script[type='application/ld+json']").unwrap();
        document
            .select(&selector)
            .map(|el| el.text().collect::<String>())
            .collect()
    }

    fn generate_issues(&self, report: &SeoReport) -> Vec<SeoIssue> {
        let mut issues = Vec::new();

        // Verificar título
        if let Some(ref title) = report.title {
            if title.len() < 30 {
                issues.push(SeoIssue {
                    severity: IssueSeverity::Warning,
                    message: "Título muito curto".to_string(),
                    recommendation: "O título deve ter pelo menos 30 caracteres".to_string(),
                });
            } else if title.len() > 60 {
                issues.push(SeoIssue {
                    severity: IssueSeverity::Warning,
                    message: "Título muito longo".to_string(),
                    recommendation: "O título deve ter no máximo 60 caracteres".to_string(),
                });
            }
        } else {
            issues.push(SeoIssue {
                severity: IssueSeverity::Critical,
                message: "Título ausente".to_string(),
                recommendation: "Adicione um título à página usando a tag <title>".to_string(),
            });
        }

        // Verificar meta description
        if let Some(ref desc) = report.meta_description {
            if desc.len() < 120 {
                issues.push(SeoIssue {
                    severity: IssueSeverity::Warning,
                    message: "Meta description muito curta".to_string(),
                    recommendation: "A meta description deve ter pelo menos 120 caracteres".to_string(),
                });
            } else if desc.len() > 160 {
                issues.push(SeoIssue {
                    severity: IssueSeverity::Warning,
                    message: "Meta description muito longa".to_string(),
                    recommendation: "A meta description deve ter no máximo 160 caracteres".to_string(),
                });
            }
        } else {
            issues.push(SeoIssue {
                severity: IssueSeverity::Critical,
                message: "Meta description ausente".to_string(),
                recommendation: "Adicione uma meta description à página".to_string(),
            });
        }

        // Verificar H1
        if report.h1_tags.is_empty() {
            issues.push(SeoIssue {
                severity: IssueSeverity::Critical,
                message: "Tag H1 ausente".to_string(),
                recommendation: "Adicione pelo menos uma tag H1 à página".to_string(),
            });
        } else if report.h1_tags.len() > 1 {
            issues.push(SeoIssue {
                severity: IssueSeverity::Warning,
                message: "Múltiplas tags H1".to_string(),
                recommendation: "Use apenas uma tag H1 por página".to_string(),
            });
        }

        // Verificar imagens sem alt
        if report.images_without_alt > 0 {
            issues.push(SeoIssue {
                severity: IssueSeverity::Warning,
                message: format!("{} imagens sem texto alternativo", report.images_without_alt),
                recommendation: "Adicione texto alternativo a todas as imagens".to_string(),
            });
        }

        issues
    }

    fn calculate_score(&self, report: &SeoReport) -> u32 {
        let mut score = 100;

        for issue in &report.issues {
            match issue.severity {
                IssueSeverity::Critical => score = score.saturating_sub(20),
                IssueSeverity::Warning => score = score.saturating_sub(10),
                IssueSeverity::Info => score = score.saturating_sub(5),
            }
        }

        score
    }
}