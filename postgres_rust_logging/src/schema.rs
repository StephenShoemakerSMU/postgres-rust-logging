table! {
    logs (log_id) {
        log_id -> Int4,
        status_code -> Nullable<Int4>,
        message -> Nullable<Text>,
        time -> Nullable<Timestamp>,
        resolved -> Nullable<Bool>,
    }
}
