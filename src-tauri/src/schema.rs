// @generated automatically by Diesel CLI.

diesel::table! {
    exopredefs (id) {
        id -> Integer,
        uuid -> Text,
        name -> Text,
    }
}

diesel::table! {
    exos (id) {
        id -> Integer,
        uuid -> Text,
        reps_exo -> Double,
        reps_rep -> Double,
        poids -> Nullable<Double>,
        exopredef_id -> Integer,
        workout_id -> Integer,
    }
}

diesel::table! {
    messages (id) {
        id -> Integer,
        uuid -> Text,
        content -> Text,
    }
}

diesel::table! {
    workouts (id) {
        id -> Integer,
        uuid -> Text,
        title -> Text,
        work_date -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    exopredefs,
    exos,
    messages,
    workouts,
);
