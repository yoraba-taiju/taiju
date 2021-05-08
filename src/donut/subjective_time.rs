#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct SubjectiveTime {
  pub leaps: u32,
  pub ticks: u32,
}
