table! {
    answer (id) {
        id -> Int4,
        content -> Varchar,
        question -> Nullable<Int4>,
        author -> Int4,
    }
}

table! {
    likes (user, answer) {
        user -> Int4,
        answer -> Int4,
    }
}

table! {
    question (id) {
        id -> Int4,
        title -> Varchar,
        content -> Varchar,
        tag_list -> Array<Text>,
        author -> Int4,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Text,
        profile_image -> Nullable<Text>,
        password_hash -> Text,
    }
}

joinable!(answer -> question (question));
joinable!(answer -> user (author));
joinable!(likes -> answer (answer));
joinable!(likes -> user (user));
joinable!(question -> user (author));

allow_tables_to_appear_in_same_query!(answer, likes, question, user,);
