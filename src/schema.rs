use juniper::{graphql_value, Executor, FieldError, FieldResult};
use juniper_from_schema::graphql_schema_from_file;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref POSTS: Mutex<HashMap<i32, Post>> = Mutex::new(HashMap::new());
    static ref CURRENT_ID: Mutex<u32> = Mutex::new(0);
}

graphql_schema_from_file!("post_schema.graphql");

pub struct Context;

impl juniper::Context for Context {}

#[derive(Clone, Debug)]
pub struct Post {
    id: i32,
    title: String,
}

impl PostFields for Post {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.id)
    }

    fn field_title(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.title)
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_get_post(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
        id: i32,
    ) -> FieldResult<Post> {
        let posts = POSTS.lock().unwrap();
        let post = posts.get(&id);
        if let Some(val) = post {
            return Ok(val.clone());
        }

        Err(FieldError::new(
            "Could not find a post",
            graphql_value!({ "internal_error": "Post not found" }),
        ))
    }

    fn field_all_posts(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
    ) -> FieldResult<Vec<Post>> {
        let posts = POSTS.lock().unwrap();
        let all_posts = posts
            .iter()
            .map(|(_, post)| post.clone())
            .collect::<Vec<Post>>();

        Ok(all_posts)
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_create_post(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
        input: CreatePost,
    ) -> FieldResult<Option<Post>> {
        let mut posts = POSTS.lock().unwrap();
        let mut id = CURRENT_ID.lock().unwrap();
        let new_id = *id as i32;
        let new_post = Post {
            id: new_id,
            title: input.title,
        };
        posts.insert(new_id, new_post.clone());
        *id += 1;

        Ok(Some(new_post))
    }

    fn field_update_post(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
        input: UpdatePost,
    ) -> FieldResult<Option<Post>> {
        let mut posts = POSTS.lock().unwrap();
        let post = posts.get(&input.id);
        if post.is_none() {
            return Err(FieldError::new(
                "Could not find a post",
                graphql_value!({ "internal_error": "Post not found" }),
            ));
        }

        let id = input.id;
        posts.entry(id).and_modify(|post| post.title = input.title);

        Ok(Some(posts.get(&id).unwrap().clone()))
    }

    fn field_delete_post(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
        id: i32,
    ) -> FieldResult<Option<Post>> {
        let mut posts = POSTS.lock().unwrap();
        let post = posts.remove(&id);
        if post.is_none() {
            return Err(FieldError::new(
                "Could not find a post",
                graphql_value!({ "internal_error": "Post not found" }),
            ));
        }

        Ok(Some(post.unwrap()))
    }
}

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
