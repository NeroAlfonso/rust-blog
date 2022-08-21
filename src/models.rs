#[derive(Queryable)] //esto se puede convertir en un row
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String
}
#[derive(Queryable, Debug)] //esto se puede convertir en un row y además poder debuggearlo
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