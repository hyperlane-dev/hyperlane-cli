mod r#enum;
mod r#fn;
mod r#impl;
mod r#struct;

#[cfg(test)]
mod test;

pub(crate) use {r#enum::*, r#fn::*, r#struct::*};
