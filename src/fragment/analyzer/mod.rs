pub mod html_parser;
pub mod boundary_detector;
pub mod candidate;
pub mod heuristic;

pub use boundary_detector::BoundaryDetector;
pub use candidate::Candidate;
pub use html_parser::{ElementInfo, HtmlParser};