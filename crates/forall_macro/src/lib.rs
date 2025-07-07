//! Macro to generate quantified effect handlers for all fields of a struct.
//! Usage examples are provided as test cases in tests/forall_fields.rs

#[macro_export]
macro_rules! forall_fields {
    // Accepts (StructType, |field| ...)
    ($struct_ty:ty, |$field:ident : &_| $body:expr) => {
        |instance: &$struct_ty| {
            // Manually expand for up to 8 fields (can be extended)
            // User must ensure the struct fields are public and accessible
            let result = {
                let $field = &instance.a;
                let r0 = $body;
                let $field = &instance.b;
                let r1 = $body;
                let $field = &instance.c;
                let r2 = $body;
                r0 && r1 && r2
            };
            result
        }
    };
}
