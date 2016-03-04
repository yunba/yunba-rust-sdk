///
///parse json
///


extern crate rustc_serialize;
extern crate regex;

pub mod json
{
    use rustc_serialize::json::Json;
    use regex::Regex;

    pub struct JsonParser
    {
        pub json_content:String,
        pub output:String
    }

    impl JsonParser
    {
        pub fn new(content:String)->JsonParser
        {
            let re=Regex::new("\\{(\n|.)*\\}").unwrap();
            let result=re.find(&content);
            let mut start=0;
            let mut end=0;
            match result
            {
                Some((x,y)) =>
                {
                    start=x;
                    end=y;

                },
                None => println!("regex match failed in json_parser_mod"),

            }
            let str_json=&content[start..end];
            JsonParser
            {
                json_content:str_json.to_string(),
                output:"".to_string()
            }
        }

        //parse json by parameter :title
        pub fn parser(&mut self,title:String)->String
        {
            let json=Json::from_str(&self.json_content).unwrap();
            let obj=json.as_object().unwrap();
            let out=obj.get(&title).unwrap().as_string();
            let mut str_content="";
            match out
            {
                 Some(str_out)=>str_content=str_out,
                 None=>println!("match fail in json_parser_mod")
            }
            return str_content.to_string();
        }
    }
}
