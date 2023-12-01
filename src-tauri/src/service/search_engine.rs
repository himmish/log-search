use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, ReloadPolicy};
use std::path::{Path, PathBuf};

// use tempfile::TempDir;

pub(crate) struct SearchEngine {
    title :Field,
    body :Field,
    index_writer: IndexWriter,
    index :Index,
}

impl SearchEngine {

    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // let index_path = TempDir::new()?;
        
        let path_str = "C:\\Users\\himan\\AppData\\LOCAL\\Temp\\query-search";
        // Convert the string to a PathBuf
        let path_buf: PathBuf = path_str.into();
        // Use AsRef<Path> to get a reference to a Path
        let path_ref: &Path = path_buf.as_ref();

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT);
        let schema = schema_builder.build();

        let title = schema.get_field("title").unwrap();
        let body = schema.get_field("body").unwrap();

        let index = Index::create_in_dir(path_ref, schema)?;
        let mut index_writer: IndexWriter = index.writer(50_000_000)?;

        Ok(SearchEngine {
            title,
            body,
            index_writer,
            index,
        })
    }

    pub(crate) fn append<S: ToString>(&mut self, file_name :S, file_content :S)  -> Result<(), Box<dyn std::error::Error>>{
        let mut old_man_doc = Document::default();

        old_man_doc.add_text(self.body, file_content);
        old_man_doc.add_text(self.title, file_name);

        self.index_writer.add_document(old_man_doc)?;

        match self.index_writer.commit() {
            Ok(result) => {
                println!("-- appended --");
                println!("response: {}", result);
                return Ok(())
            },
            Err(err) => {
                println!("{}", err);
                return Err(Box::new(err))
            },
        };
    }

    pub(crate) fn search(&self, q: &str)  -> Result<Vec<String>, Box<dyn std::error::Error>>{        
        println!("query : {}", q);

        let reader = self.index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        let searcher = reader.searcher();
        
        let query_parser = QueryParser::for_index(&self.index, vec![self.title, self.body]);
        let query = query_parser.parse_query(q)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
        println!("-- searching --");
        println!("{}", top_docs.len());

        let mut res: Vec<String> = Vec::new();

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            println!("score: {}", _score);
            let title = retrieved_doc
                .get_first(self.title)
                .and_then(|v| v.as_text().map(|f| f.to_string()))
                .unwrap_or_else(|| String::new());

            res.push(title);
            dbg!(retrieved_doc);
        }

        println!("-- searched --");
        Ok(res)
    }
}