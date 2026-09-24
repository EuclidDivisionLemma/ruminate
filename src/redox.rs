use crate::Os;

pub struct Redox;

pub static OS: Redox = Redox;

impl Os for Redox {}
