type Solution = fn(&str) -> (i64, i64);

macro_rules! solutions {
    ([$($module:ident),*$(,)?]) => {
        $(
            mod $module;
        )*

        pub static REGISTRY: &[Solution] = &[
            $($module::solution,)*
        ];
    }
}

solutions!([day_1, day_2, day_3, day_4]);
