#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


use serde::Serialize;
use serde::Deserialize;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostSummary{

    //pub id: u32,
    pub title: String,
    pub summary: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostContent{

    pub content: String,

}

impl PostContent{

    pub fn default() -> PostContent{

        PostContent{ content: String::default() }
    }

}





#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DykRatings{

    //the order of dyk id's and the rating
    ratings: Vec<(String, u32)>,

    //pub skipped: Vec<String>,

}

impl DykRatings{

    pub fn new() -> DykRatings{

        DykRatings{

            ratings: Vec::new(),

        }

    }

    pub fn add_rating(&mut self, id: String, rating: u32){

        if self.has_already_seen( &id ){

        }
        else{
            self.ratings.push( (id, rating) );
        }

    }

    pub fn has_already_seen( &self, id: &String) -> bool{

        let mut hashset: HashSet<&String> = self.ratings.iter().map(|(id,rat)|  id  ).collect();

        //hashset.extend::<HashSet<&String>>(  self.skipped.iter().collect() );

        hashset.contains( id )

    }

    /*
    pub fn unsee_and_unrate(&mut self, id:&String){


    }
    */

    pub fn pop_most_recently_rated(&mut self) -> Option<String>{

        if let Some( (id, _) ) = self.ratings.pop() {

            return Some(id);
        }
        else{
            return None;
        };


    }


}


use std::hash::Hash;

/*
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct DykID{

    pub ID: String,
}

*/









use std::collections::HashMap;
use std::collections::HashSet;

pub struct DidYouKnowContent{

    //the id to the content
    pub posts: HashMap<String, String>,

    //the categories to ids
    pub categories: HashMap<PostCategory, HashSet<String> >,

    
}

impl DidYouKnowContent{

    pub fn get_category_of_postid(&self, id: &str) -> PostCategory{

        for (category, posts) in &self.categories{
            if posts.contains( id ){
                return category.clone();
            }
        }

        panic!("no category for this post with this id");
    }


    //
    pub fn get_posts_in_category(&self, category: &PostCategory ) -> &HashSet<String>{

        self.categories.get(category).unwrap()

    }


    fn does_post_exist(&self, id: &String) -> bool{

        if let Some(_) = self.posts.get(id ){
            return true;
        }
        else{
            return false;
        }

    }


    pub fn get_post(&self, id: &String) -> Option< String >{

        if let Some(post) = self.posts.get( id ){
            return Some(post.clone());
        }
        else{
            return None;
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PostCategory{

    Comics,
    Nonsense,
    Fiction,
    Rhyming,
    Philosophical,

}


impl PostCategory{

    pub fn letter_to_category(letter: &str) -> Option<PostCategory>{

        match letter{

            "C" => Some(PostCategory::Comics),
            "N" => Some(PostCategory::Nonsense),
            "F" => Some(PostCategory::Fiction),
            "R" => Some(PostCategory::Rhyming),
            "P" => Some(PostCategory::Philosophical),
            _ => None,

        }
    }

    pub fn get_all() -> Vec<PostCategory>{

        let mut toreturn = Vec::new();

        toreturn.push( PostCategory::Comics );
        toreturn.push( PostCategory::Fiction );
        //toreturn.push( PostCategory::Nonsense );
        //toreturn.push( PostCategory::Rhyming );
        toreturn.push( PostCategory::Philosophical );


        return toreturn;
    }


    pub fn get_name(&self) -> &str{

        match self{

            PostCategory::Comics => "comics i will never draw",
            PostCategory::Nonsense => "nonsense",
            PostCategory::Fiction => "fiction",
            PostCategory::Rhyming => "rhyming",
            PostCategory::Philosophical => "political/philosophical",

        }

    }


}

