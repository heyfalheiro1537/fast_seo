# 🚀 SEO-RS

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


Uma biblioteca Rust poderosa e moderna para **análise e otimização de SEO**. Analise páginas web, gere relatórios detalhados e obtenha recomendações práticas para melhorar o posicionamento nos mecanismos de busca.

## ✨ Funcionalidades

- 🔍 **Análise Completa de Páginas**: Títulos, meta descriptions, estrutura de headings
- 📊 **Sistema de Pontuação**: Score de 0-100 baseado em melhores práticas de SEO  
- 🏷️ **Meta Tags & Open Graph**: Verificação completa de metadados sociais
- 🗺️ **Gerador de Sitemap**: Criação e análise de sitemaps XML
- 📈 **Densidade de Palavras-chave**: Análise automática de densidade
- 🖼️ **Otimização de Imagens**: Detecção de imagens sem alt text
- 🔗 **Análise de Links**: Contagem de links internos e externos
- ⚡ **Performance**: Medição de tempo de carregamento
- 📋 **Relatórios Detalhados**: Problemas categorizados com recomendações

## 🚀 Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
seo-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 📖 Uso Básico

```rust
use seo_rs::analyzer::SeoAnalyzer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = SeoAnalyzer::new();
    let report = analyzer.analyze_url("https://example.com").await?;
    
    println!("🎯 Score SEO: {}/100", report.score);
    println!("📄 Título: {:?}", report.title);
    println!("📝 Meta Description: {:?}", report.meta_description);
    println!("🏷️ Tags H1: {:?}", report.h1_tags);
    
    // Exibir problemas encontrados
    for issue in &report.issues {
        println!("{:?}: {}", issue.severity, issue.message);
        println!("💡 {}", issue.recommendation);
    }
    
    Ok(())
}
```

## 🔧 Funcionalidades Avançadas

### Análise de Meta Tags

```rust
use seo_rs::meta::MetaAnalyzer;
use scraper::Html;

let html = Html::parse_document(&html_content);
let meta_tags = MetaAnalyzer::extract_all_meta_tags(&html);
let og_tags = MetaAnalyzer::check_open_graph(&html);

println!("Meta tags encontradas: {}", meta_tags.len());
println!("Open Graph tags: {:?}", og_tags);
```

### Geração de Sitemap

```rust
use seo_rs::sitemap::{SitemapGenerator, SitemapUrl};

let urls = vec![
    SitemapUrl {
        loc: "https://example.com".to_string(),
        lastmod: Some("2025-01-01".to_string()),
        changefreq: Some("daily".to_string()),
        priority: Some(1.0),
    },
    SitemapUrl {
        loc: "https://example.com/about".to_string(),
        lastmod: Some("2024-12-15".to_string()),
        changefreq: Some("monthly".to_string()),
        priority: Some(0.8),
    },
];

let xml = SitemapGenerator::generate_xml(&urls);
println!("{}", xml);
```

### Relatório Detalhado

```rust
use seo_rs::analyzer::SeoAnalyzer;
use seo_rs::IssueSeverity;

let analyzer = SeoAnalyzer::new();
let report = analyzer.analyze_url("https://seu-site.com").await?;

println!("=== RELATÓRIO SEO COMPLETO ===");
println!("URL: {}", report.url);
println!("Score: {}/100", report.score);
println!("Tempo de carregamento: {:.2}s", report.load_time.unwrap_or(0.0));
println!("Tamanho da página: {} bytes", report.page_size.unwrap_or(0));

// Estatísticas
println!("\n📊 ESTATÍSTICAS:");
println!("• Links internos: {}", report.internal_links);
println!("• Links externos: {}", report.external_links);
println!("• Imagens sem alt: {}", report.images_without_alt);
println!("• Tags H2: {}", report.h2_tags.len());

// Problemas por categoria
let critical = report.issues.iter().filter(|i| matches!(i.severity, IssueSeverity::Critical)).count();
let warnings = report.issues.iter().filter(|i| matches!(i.severity, IssueSeverity::Warning)).count();

println!("\n🚨 PROBLEMAS:");
println!("• Críticos: {}", critical);
println!("• Avisos: {}", warnings);

// Top palavras-chave
println!("\n🔑 TOP PALAVRAS-CHAVE:");
let mut keywords: Vec<_> = report.keyword_density.iter().collect();
keywords.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
for (word, density) in keywords.iter().take(10) {
    println!("• {}: {:.2}%", word, density);
}
```

## 📁 Estrutura do Projeto

```
seo-rs/
├── src/
│   ├── lib.rs              # Módulo principal e tipos
│   ├── analyzer.rs         # Analisador principal de SEO
│   ├── meta.rs            # Análise de meta tags
│   ├── sitemap.rs         # Geração e análise de sitemaps
│   ├── keywords.rs        # Análise de palavras-chave
│   ├── performance.rs     # Métricas de performance
│   └── structured_data.rs # Dados estruturados (JSON-LD)
├── examples/
│   ├── basic_usage.rs     # Exemplo básico
│   ├── batch_analysis.rs  # Análise em lote
│   └── sitemap_gen.rs     # Geração de sitemap
└── tests/
    └── integration_tests.rs
```

## 🎯 Tipos de Problemas Detectados

### Críticos ❌
- Título ausente ou vazio
- Meta description ausente
- Tag H1 ausente
- Múltiplas tags H1

### Avisos ⚠️
- Título muito curto/longo (< 30 ou > 60 caracteres)
- Meta description muito curta/longa (< 120 ou > 160 caracteres)
- Imagens sem texto alternativo
- Poucos links internos

### Informativos ℹ️
- Sugestões de otimização
- Recomendações de estrutura
- Dicas de performance

## 🧪 Executando os Exemplos

```bash
# Análise básica
cargo run --example basic_usage

# Análise em lote de URLs
cargo run --example batch_analysis

# Geração de sitemap
cargo run --example sitemap_gen
```

## 🧪 Testes

```bash
# Executar todos os testes
cargo test

# Testes com output detalhado
cargo test -- --nocapture

# Executar apenas testes de integração
cargo test --test integration_tests
```

## 🛠️ Desenvolvimento

### Pré-requisitos
- Rust 1.70+
- Tokio runtime para operações assíncronas

### Configuração do ambiente
```bash
git clone https://github.com/seuusuario/seo-rs.git
cd seo-rs
cargo build
cargo test
```

### Contribuindo

1. 🍴 Fork o projeto
2. 🌟 Crie sua feature branch (`git checkout -b feature/MinhaFeature`)
3. ✅ Adicione testes para suas mudanças
4. ✏️ Commit suas mudanças (`git commit -m 'feat: adiciona MinhaFeature'`)
5. 📤 Push para a branch (`git push origin feature/MinhaFeature`)
6. 🎯 Abra um Pull Request

### Padrão de Commits
- `feat:` nova funcionalidade
- `fix:` correção de bug
- `docs:` mudanças na documentação
- `test:` adicionar ou modificar testes
- `refactor:` refatoração de código

## 🗺️ Roadmap

- [ ] 🔍 **v0.2.0**: Análise de Core Web Vitals
- [ ] 🤖 **v0.3.0**: Análise de robots.txt e canonical URLs
- [ ] 📊 **v0.4.0**: Relatórios em HTML/PDF
- [ ] 🕷️ **v0.5.0**: Crawler para análise de sites completos
- [ ] 🧠 **v1.0.0**: Análise de legibilidade e estrutura semântica
- [ ] ☁️ **v1.1.0**: Integração com APIs de SEO (Google Search Console)

## 📚 Recursos Adicionais

TBD

## 📋 Checklist SEO Implementado

### ✅ Análise Básica
- [x] Título da página
- [x] Meta description
- [x] Tags de cabeçalho (H1-H6)
- [x] Meta tags básicas
- [x] Open Graph tags

### ✅ Análise de Conteúdo
- [x] Densidade de palavras-chave
- [x] Estrutura de headings
- [x] Análise de links
- [x] Imagens e alt text

### ✅ Análise Técnica
- [x] Tempo de carregamento
- [x] Tamanho da página
- [x] Estrutura de dados JSON-LD
- [x] Geração de sitemap

### 🔄 Em Desenvolvimento
- [ ] Análise de Core Web Vitals
- [ ] Verificação de canonical URLs
- [ ] Análise de schema markup
- [ ] Verificação de mobile-friendly

## ⚡ Performance

A biblioteca foi otimizada para performance:

- 🚄 **Processamento assíncrono** com Tokio
- 🧠 **Parsing eficiente** com scraper/html5ever  
- 💾 **Baixo uso de memória** com processamento streaming
- ⚡ **Análise paralela** de múltiplas URLs

### Benchmarks

TBD

## 🤝 Comunidade

- 📧 Email: heyfalheiro@gmail.com

## 📄 Licença

Este projeto está licenciado sob a [Licença MIT](LICENSE) - veja o arquivo LICENSE para detalhes.

## 🙏 Agradecimentos

- [scraper](https://github.com/causal-agent/scraper) - HTML parsing
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [tokio](https://tokio.rs/) - Runtime assíncrono
- Comunidade Rust 🦀

---

<div align="center">

**Desenvolvido com ❤️ em Rust**

[⭐ Star no GitHub](https://github.com/heyfalheiro1537/fast_seo) • [📦 Crates.io](https://crates.io/crates/seo-rs) • [📚 Docs](https://docs.rs/seo-rs)

</div>
