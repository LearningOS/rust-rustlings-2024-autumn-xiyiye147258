// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub trait Grade {
    fn to_string(&self) -> String;
}

// 为f32类型的成绩实现Grade特征
impl Grade for f32 {
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}

// 为String类型的成绩实现Grade特征
impl Grade for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}
pub struct ReportCard<G: Grade> {
    pub grade: G,
    pub student_name: String,
    pub student_age: u8,
}

impl<G: Grade> ReportCard<G> {
    pub fn print(&self) -> String {
        // 使用泛型G的to_string方法来获取成绩的字符串表示
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, self.grade.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // 使用字母成绩"A+"
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
