use log::debug;
/// The actual implementation of the `Nop` preprocessor. This would usually go
/// in your main `lib.rs` file.
    use mdbook::{preprocess::{Preprocessor, PreprocessorContext}, book::Book, BookItem};
    use pulldown_cmark::{Parser, Event, CodeBlockKind, Tag, CowStr};


/// A Preprocessor for testing YAML code blocks
pub struct YamlTester;

impl YamlTester {
    pub fn new() -> YamlTester {
        YamlTester
    }
}

impl Preprocessor for YamlTester {
    fn name(&self) -> &str {
        "lace-yaml-tester"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> anyhow::Result<Book> {
        debug!("Starting the run");
        for book_item in book.iter() {
            // debug!("Examining item {:?}\n", book_item);
            if let BookItem::Chapter(chapter) = book_item {
                debug!("Examining Chapter {}", chapter.name);
                let parser = Parser::new(&chapter.content);
                let mut code_block = Some(String::new());
                for event in parser {
                    // if let Event::Code(content) = event {
                    //     debug!("Found code: {}", content);
                    // }
                    if let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref code_block_string))) = event {
                        if code_block_string == &CowStr::from("yaml") {
                            debug!("YAML Block Start, string={}", code_block_string);
                            code_block=Some(String::new());    
                        }
                    } else if let Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(ref code_block_string))) = event {
                        if code_block_string == &CowStr::from("yaml") {
                            debug!("Code Block End, string={}", code_block_string);
                            let final_block = code_block.take();
                            debug!("Code block ended up as\n{}", final_block.unwrap_or("<NO STRING FOUND>".to_string()));
                        }
                    } else if let Event::Text(ref text) = event {
                        if let Some(existing) = code_block.as_mut() {
                            existing.push_str(text);
                        }
                        
                        ;
                    }
                }
            }
        }

        Ok(book)
    }
}


//     fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
//         // In testing we want to tell the preprocessor to blow up by setting a
//         // particular config value
//         if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
//             if nop_cfg.contains_key("blow-up") {
//                 anyhow::bail!("Boom!!1!");
//             }
//         }

//         // we *are* a no-op preprocessor after all
//         Ok(book)
//     }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn nop_preprocessor_run() {
//         let input_json = r##"[
//             {
//                 "root": "/path/to/book",
//                 "config": {
//                     "book": {
//                         "authors": ["AUTHOR"],
//                         "language": "en",
//                         "multilingual": false,
//                         "src": "src",
//                         "title": "TITLE"
//                     },
//                     "preprocessor": {
//                         "nop": {}
//                     }
//                 },
//                 "renderer": "html",
//                 "mdbook_version": "0.4.21"
//             },
//             {
//                 "sections": [
//                     {
//                         "Chapter": {
//                             "name": "Chapter 1",
//                             "content": "# Chapter 1\n",
//                             "number": [1],
//                             "sub_items": [],
//                             "path": "chapter_1.md",
//                             "source_path": "chapter_1.md",
//                             "parent_names": []
//                         }
//                     }
//                 ],
//                 "__non_exhaustive": null
//             }
//         ]"##;
//         let input_json = input_json.as_bytes();

//         let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
//         let expected_book = book.clone();
//         let result = Nop::new().run(&ctx, book);
//         assert!(result.is_ok());

//         // The nop-preprocessor should not have made any changes to the book content.
//         let actual_book = result.unwrap();
//         assert_eq!(actual_book, expected_book);
//     }
// }
