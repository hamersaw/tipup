use bson::ordered::OrderedDocument;

use analyzer::Analyzer;
use error::TipupError;
use flag_manager::Flag;

use std::sync::mpsc::Sender;

pub struct ErrorAnalyzer {
    tx: Sender<Flag>,
}

impl ErrorAnalyzer {
    pub fn new(_: &OrderedDocument, tx: Sender<Flag>) -> Result<ErrorAnalyzer, TipupError> {
        Ok(
            ErrorAnalyzer {
                tx: tx,
            }
        )
    }
}

impl Analyzer for ErrorAnalyzer {
    fn add_result(&self) -> Result<(), TipupError> {
        unimplemented!();
    }
}
