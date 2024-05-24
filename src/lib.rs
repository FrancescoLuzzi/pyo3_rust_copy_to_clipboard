// rustimport:pyo3

use arboard::Clipboard as AClipboard;
use nucleo_matcher::{
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
    Config, Matcher as NucleoMatcher,
};
use pyo3::{
    exceptions::{PyFileNotFoundError, PyValueError},
    prelude::*,
};

use infer::{Infer, MatcherType, Type};

#[pyclass(module = "pyo3_rust")]
#[derive(Debug, Clone)]
struct Matcher {
    matcher: NucleoMatcher,
    pattern: Pattern,
}

#[pymethods]
impl Matcher {
    #[new]
    fn new(pattern: String) -> Self {
        Matcher {
            matcher: NucleoMatcher::new(Config::DEFAULT),
            pattern: Pattern::new(
                &pattern,
                CaseMatching::Smart,
                Normalization::Smart,
                AtomKind::Fuzzy,
            ),
        }
    }

    fn set_pattern(&mut self, new_pattern: &str) {
        self.pattern
            .reparse(new_pattern, CaseMatching::Smart, Normalization::Smart);
    }

    fn match_list(&mut self, elements: Vec<String>) -> Vec<(String, u32)> {
        self.pattern.match_list(elements, &mut self.matcher)
    }

    fn match_list_top_n(&mut self, elements: Vec<String>, num_elements: usize) -> Vec<String> {
        self.pattern
            .match_list(elements, &mut self.matcher)
            .into_iter()
            .map(|x| x.0)
            .take(num_elements)
            .collect()
    }
}

#[pyclass(module = "pyo3_rust")]
struct Clipboard {
    clipboard: AClipboard,
    infer: Infer,
    last_infer: Option<Type>,
}

#[pyclass(module = "pyo3_rust")]
enum ClipboardContent {
    Text { content: String },
    Html { content: String },
    Image { content: Vec<u8> },
}

const ACCEPTED_EXTENSIONS: [&str; 6] = ["txt", "rtf", "md", "bat", "ps1", "psm"];

#[pymethods]
impl Clipboard {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            clipboard: AClipboard::new().map_err(|err| PyValueError::new_err(err.to_string()))?,
            infer: Infer::new(),
            last_infer: None,
        })
    }

    fn set_from_path(&mut self, path: &str) -> PyResult<()> {
        let file_content = std::fs::read(path).map_err(PyFileNotFoundError::new_err)?;
        let mime_match = self.infer.get(&file_content);
        if mime_match.is_none() {
            let ext = std::path::Path::new(path)
                .extension()
                .ok_or(PyValueError::new_err("mime type not found"))?;
            if ACCEPTED_EXTENSIONS.iter().any(|&x| x == ext) {
                let content = String::from_utf8(file_content)
                    .map_err(|err| PyValueError::new_err(err.to_string()))?;
                self.clipboard
                    .set_text(&content)
                    .map_err(|err| PyValueError::new_err(err.to_string()))?
            };
            return Ok(());
        }

        let mime_match = mime_match.unwrap();
        match (mime_match.matcher_type(), mime_match.mime_type()) {
            (MatcherType::Text, "text/html") => {
                let content = String::from_utf8(file_content).map_err(PyValueError::new_err)?;
                self.clipboard
                    .set_html(&content, Some(&content))
                    .map_err(|err| PyValueError::new_err(err.to_string()))?
            }
            (MatcherType::Text, "text/x-shellscript") => {
                let content = String::from_utf8(file_content).map_err(PyValueError::new_err)?;
                self.clipboard
                    .set_text(&content)
                    .map_err(|err| PyValueError::new_err(err.to_string()))?
            }
            // TODO: extract size
            // (MatcherType::Image, _) => {
            //     let img_content = ImageData{ width: todo!(), height: todo!(), bytes: todo!() }};
            //     self
            //                 .clipboard
            //                 .set_image(&file_content)
            //                 .map_err(|err| PyValueError::new_err(err))?
            // }
            (mime, _) => Err(PyValueError::new_err(format!(
                "mime type not supported: {mime:?}"
            )))?,
        };
        Ok(())
    }
}

#[pymodule]
fn pyo3_rust(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<Matcher>()?;
    m.add_class::<Clipboard>()?;
    m.add_class::<ClipboardContent>()?;
    Ok(())
}
