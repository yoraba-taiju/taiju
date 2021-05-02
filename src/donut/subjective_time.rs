use super::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub struct SubjectiveTime {
  pub(crate) leaps: u32,
  pub(crate) ticks: u32,
}

impl SubjectiveTime {
  pub(crate) fn new(leaps:u32, ticks:u32) -> Self {
    Self {
      leaps,
      ticks,
    }
  }
}
