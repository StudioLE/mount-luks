use crate::prelude::*;

#[derive(Clone)]
pub struct AnyReport {
    rendered: String,
}

impl<T: StdError + Send + Sync + 'static> From<Report<T>> for AnyReport {
    fn from(report: Report<T>) -> Self {
        Self {
            rendered: report.render(),
        }
    }
}

impl Debug for AnyReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.rendered)
    }
}

impl Display for AnyReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.rendered)
    }
}
