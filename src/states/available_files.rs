use crate::models::MetricFileHttpModel;

pub struct AvailableFiles {
    files: Option<Vec<MetricFileHttpModel>>,
}

impl AvailableFiles {
    pub fn new() -> Self {
        Self { files: None }
    }

    pub fn initialized(&self) -> bool {
        self.files.is_some()
    }
    pub fn reset(&mut self) {
        self.files = None;
    }

    pub fn set_files(&mut self, files: Vec<MetricFileHttpModel>) {
        self.files = Some(files);
    }

    pub fn get_files(&self) -> Option<&Vec<MetricFileHttpModel>> {
        self.files.as_ref()
    }

    pub fn get_available_hours_ago(&self, hours_ago: i64) -> Option<i64> {
        let files = self.files.as_ref()?;

        for file in files {
            if file.hours_ago == hours_ago {
                return Some(hours_ago);
            }
        }

        Some(files.get(0)?.hours_ago)
    }
}
