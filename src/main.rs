use rust_search::SearchBuilder;

fn main() {
    fn inputs() {
        let file_type: Vec<String> = SearchBuilder::default()
            .ext("gin")
            .build()
            .collect();
        std::mem::drop(file_type); // Correct: drops the variable here
    }
    inputs();
    fn syntax() { 
    }
}

