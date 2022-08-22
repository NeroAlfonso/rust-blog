use serde::{Deserialize, Serialize};
use diesel::prelude::*;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler
{
    pub title: String,
    pub body: String
}

#[derive(Queryable, Debug, Deserialize, Serialize)] //esto se puede convertir en un row
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String
}
#[derive(Queryable, Debug, Deserialize, Serialize)] //esto se puede convertir en un row y además poder debuggearlo
pub struct PostTitleAndSlug
{
    pub title: String,
    pub slug: String
}
use super::schema::posts;
#[derive(Insertable)] //macro
#[table_name="posts"] //macro
pub struct NewPost<'a> //poder utilizar los std dentro de las estructuras (lifetime)
{
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str
}

impl Post {
    pub fn slugify(title: &String) -> String 
    {
        title.replace(title, "-").to_lowercase()
    }
    pub fn create_post<'a>
    (conn: &PgConnection, post: &NewPostHandler) -> Result<Post, diesel::result::Error>
    {
        let slug = Post::slugify(&post.title.clone());
        let new_post = NewPost{
            title: &post.title,
            slug: &slug,
            body: &post.body
        };
        diesel::insert_into(posts::table).values(new_post).get_result::<Post>(conn)
    }
}