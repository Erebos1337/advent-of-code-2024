pub fn remove_element(report: &[u32], index: usize) -> Vec<u32> {
    let mut new_report = report[..index].to_vec();
    new_report.extend_from_slice(&report[index + 1..]);
    new_report
}