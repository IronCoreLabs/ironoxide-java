mod swig_foreign_types_map {}

foreign_typemap!(
    ($p:r_type) DateTime<Utc> => i64 {
        $out = $p.timestamp_millis();
    };
);

// foreign_typemap!(
//     ($p:r_type) Option<DateTime<Utc>> => internal_aliases::JOptionalLong {
//         let tmp: Option<i64> = $p.map(|x| x.timestamp_millis());
//         $out = to_java_util_optional_long(env, tmp);
//     };
// );
