pub struct LogData {
    pub node_num: usize,
    pub condition: char,
    pub filename: String,
    pub line: usize
}

pub type LogList = Vec<LogData>;
