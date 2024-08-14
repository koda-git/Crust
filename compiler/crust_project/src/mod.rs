// Added the source code struct to manage the source code contents

pub struct SourceCode {
    source_code: String,
    source_name: String,
    new_line_indices: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceIndex {
    index: usize,
    line: usize,
    column: usize,
}

#[test]
fn test_source_index() {
    let source_index = SourceIndex {
        index: 0,
        line: 1,
        column: 1,
    };
    assert_eq!(source_index.index, 0);
    assert_eq!(source_index.line, 1);
    assert_eq!(source_index.column, 1);
}

impl SourceCode {
    pub fn new(source_code: String, source_name: String) -> Self {
        let mut new_line_indices = vec![0];
        for (index, character) in source_code.chars().enumerate() {
            if character == '\n' {
                new_line_indices.push(index + 1);
            }
        }
        Self {
            source_code,
            source_name,
            new_line_indices,
        }
    }

    pub fn get_source_name(&self) -> &str {
        &self.source_name
    }

    pub fn get_source_code(&self) -> &str {
        &self.source_code
    }

    pub fn get_source_index(&self, index: usize) -> SourceIndex {
        let line = self
            .new_line_indices
            .binary_search(&index)
            .unwrap_or_else(|x| x)
            + 1;
        let column = index - self.new_line_indices[line - 1] + 1;
        SourceIndex {
            index,
            line,
            column,
        }
    }
}
