use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, self};
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, ReloadPolicy};
use std::path::{Path, PathBuf};

// use tempfile::TempDir;

pub(crate) struct SearchEngine {
    name :Field,
    body :Field,
    path: Field,
    last_modified: Field,
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

        let mut schema_builder: SchemaBuilder = Schema::builder();
        
        schema_builder.add_text_field("name", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT);
        schema_builder.add_text_field("path", TEXT | STORED);
        schema_builder.add_text_field("last_modified", TEXT | STORED);
        
        let schema = schema_builder.build();

        let name: Field = schema.get_field("name").unwrap();
        let body: Field = schema.get_field("body").unwrap();
        let path: Field = schema.get_field("path").unwrap();
        let last_modified: Field = schema.get_field("last_modified").unwrap();

        let index = Index::create_in_dir(path_ref, schema)?;
        let mut index_writer: IndexWriter = index.writer(50_000_000)?;

        Ok(SearchEngine {
            name,
            body,
            path,
            last_modified,
            index_writer,
            index,
        })
    }

    pub(crate) fn append<S: ToString>(&mut self, name :S, path: S, content :S, last_modified: u64)  -> Result<(), Box<dyn std::error::Error>>{
        

        let mut document_already_exist = false;
        match self.search(&path.to_string()) {
            Ok(res) => document_already_exist = res.into_iter().any(|t| t == name.to_string()),
            Err(err) => println!("{:?}", err),
        } 
        if document_already_exist { 
            println!("document already indexed no need to append");
            return Ok(()) 
        }
        
        let mut old_man_doc = Document::default();

        old_man_doc.add_text(self.name, name);
        old_man_doc.add_text(self.body, content);
        old_man_doc.add_text(self.path, path);
        old_man_doc.add_text(self.last_modified, last_modified);

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
        
        let query_parser = QueryParser::for_index(&self.index, vec![self.name, self.body]);
        let query = query_parser.parse_query(q)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
        println!("-- searching --");
        println!("{}", top_docs.len());

        let mut res: Vec<String> = Vec::new();

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            println!("score: {}", _score);
            let name = retrieved_doc
                .get_first(self.name)
                .and_then(|v| v.as_text().map(|f| f.to_string()))
                .unwrap_or_else(|| String::new());

            res.push(name);
            dbg!(retrieved_doc);
        }

        println!("-- searched --");
        Ok(res)
    }
}