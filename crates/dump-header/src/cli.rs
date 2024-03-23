use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum Lang {
    C,
    #[value(name = "objective-c", alias = "objc")]
    ObjC,
    //    #[value(name = "c++", alias = "cpp")]
    //    Cpp,
}

#[derive(ValueEnum, Clone)]
pub enum Std {
    C90,
    GNU90,
    C99,
    GNU99,
    C11,
    GNU11,
    C17,
    GNU17,
}
