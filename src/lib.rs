#[macro_export]
#[rustfmt::skip]
macro_rules! vargs {
    (
        $(#[$struct_attrs:meta])*
        $Name:ident $(<$($L:lifetime),+>)* {
            $(
                $(@[$option:tt])*
                $(#[$arg_attrs:meta])*
                $arg:ident: $Type:ty
            ),*
        }
    ) => {
        $(#[$struct_attrs])*
        pub struct $Name $(<$($L),+>)* {
            $(
                $(#[$arg_attrs])*
                pub $arg: vargs!(@field; $($option)* $Type)
            ),*
        }

        impl $(<$($L),+>)* From<Option<$Name $(<$($L),+>)*>> for $Name $(<$($L),+>)* {
            fn from(opt: Option<$Name $(<$($L),+>)*>) -> $Name $(<$($L),+>)* {
                opt.unwrap_or_default()
            }
        }

        impl $(<$($L),+>)* From<()> for $Name $(<$($L),+>)* {
            fn from(_: ()) -> $Name $(<$($L),+>)* {
                Default::default()
            }
        }

        vargs!(@vargs; $Name $(<$($L),+>)* { $($arg: $Type),* });
    };
    (@field; option $Type:ty) => {
        Option<$Type>
    };
    (@field; $Type:ty) => {
        $Type
    };
    (@from; $Name:ident $(<$($L:lifetime),+>)* { $($arg:ident: $Type:ty),* }) => {
        impl $(<$($L),+>)* From<($($Type),*)> for $Name $(<$($L),+>)* {
            #[allow(unused_parens)]
            fn from(($($arg),*) : ($($Type),*)) -> $Name $(<$($L),+>)* {
                $Name {
                    $(
                        $arg: $arg.into(),
                    )*
                    ..Default::default()
                }
            }
        }
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty, $arg2:ident: $Type2:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2 });
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty, $arg2:ident: $Type2:ty, $arg3:ident: $Type3:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3 });
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty, $arg2:ident: $Type2:ty, $arg3:ident: $Type3:ty, $arg4:ident: $Type4:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4 });
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty, $arg2:ident: $Type2:ty, $arg3:ident: $Type3:ty, $arg4:ident: $Type4:ty, $arg5:ident: $Type5:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5 });
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty, $arg2:ident: $Type2:ty, $arg3:ident: $Type3:ty, $arg4:ident: $Type4:ty, $arg5:ident: $Type5:ty, $arg6:ident: $Type6:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6 });
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty, $arg2:ident: $Type2:ty, $arg3:ident: $Type3:ty, $arg4:ident: $Type4:ty, $arg5:ident: $Type5:ty, $arg6:ident: $Type6:ty, $arg7:ident: $Type7:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7 });
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty, $arg2:ident: $Type2:ty, $arg3:ident: $Type3:ty, $arg4:ident: $Type4:ty, $arg5:ident: $Type5:ty, $arg6:ident: $Type6:ty, $arg7:ident: $Type7:ty, $arg8:ident: $Type8:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7, $arg8: $Type8 });
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty, $arg2:ident: $Type2:ty, $arg3:ident: $Type3:ty, $arg4:ident: $Type4:ty, $arg5:ident: $Type5:ty, $arg6:ident: $Type6:ty, $arg7:ident: $Type7:ty, $arg8:ident: $Type8:ty, $arg9:ident: $Type9:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7, $arg8: $Type8 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7, $arg8: $Type8, $arg9: $Type9 });
    };
    (@vargs; $Name:ident $(<$($L:lifetime),+>)* { $arg1:ident: $Type1:ty, $arg2:ident: $Type2:ty, $arg3:ident: $Type3:ty, $arg4:ident: $Type4:ty, $arg5:ident: $Type5:ty, $arg6:ident: $Type6:ty, $arg7:ident: $Type7:ty, $arg8:ident: $Type8:ty, $arg9:ident: $Type9:ty, $arg10:ident: $Type10:ty }) => {
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7, $arg8: $Type8 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7, $arg8: $Type8, $arg9: $Type9 });
        vargs!(@from; $Name $(<$($L),+>)* { $arg1: $Type1, $arg2: $Type2, $arg3: $Type3, $arg4: $Type4, $arg5: $Type5, $arg6: $Type6, $arg7: $Type7, $arg8: $Type8, $arg9: $Type9, $arg10: $Type10 });
    };
}
