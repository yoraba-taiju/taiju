#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub struct SubjectiveTime {
  pub leaps: u32,
  pub ticks: u32,
}

impl SubjectiveTime {
  #[allow(dead_code)]
  pub fn new(leaps:u32, ticks:u32) -> Self {
    Self {
      leaps,
      ticks,
    }
  }
}
