/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use paste::paste;
use uniffi_bindgen::backend::{CodeType, Literal};

macro_rules! impl_code_type_for_miscellany {
    ($T:ty, $class_name:literal, $canonical_name:literal) => {
        paste! {
            #[derive(Clone, Debug)]
            pub struct $T;

            impl CodeType for $T  {
                fn type_label(&self) -> String {
                    $class_name.into()
                }

                fn canonical_name(&self) -> String {
                   $canonical_name.into()
               }

                fn literal(&self, _literal: &Literal) -> String {
                    unreachable!()
                }
            }
        }
    };
}

impl_code_type_for_miscellany!(TimestampCodeType, "DateTime", "Timestamp");

impl_code_type_for_miscellany!(DurationCodeType, "TimeSpan", "Duration");
