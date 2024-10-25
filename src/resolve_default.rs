/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

/// Generate an Option<T> by chaining together an Option<T> with an Option<struct { field: Option<T> }>
#[macro_export]
macro_rules! resolve_default (
    ( $value:ident , $default_struct:ident . $default_field:ident ) => {
        $value
            .as_ref()
            .map(|x| x.clone())
            .or(
                $default_struct
                    .and_then(|d| {
                        d.$default_field
                            .as_ref()
                            .map(|x| x.clone())
                    })
            )
    }
);
