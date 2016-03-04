///
///convert request msg
///



pub mod register_ticket
{
    use std::str::from_utf8;

    pub struct RegisterTicketConfig
    {
        version:usize,
        json:String,
        length:usize
    }

    //convert usize to u8
    pub fn parse_num(num:usize) -> String
    {
        let judge_num:i32=num as i32;
        let num_u16=num as u16;
        let num_little=num_u16.to_le();
        let low:u8=(num_little & 0x00ff) as u8;
        let high:u8=((num_little>>8) & 0x00ff) as u8;
        if judge_num>=0 && judge_num <=9
        {
            let mut bit_vec:Vec<u8>=Vec::new();
            bit_vec.push(low);
            let bit_str=String::from_utf8(bit_vec).unwrap();
            return bit_str;
        }
        else if judge_num>9 && judge_num<255
        {
            let mut bit_vec:Vec<u8>=Vec::new();
            bit_vec.push(high);
            bit_vec.push(low);
            let bit_str=String::from_utf8(bit_vec).unwrap();
            return bit_str;
        }
        else
        {
            return "".to_string();
        }
    }

    impl RegisterTicketConfig
    {
        pub fn new(version:usize,json:String,length:usize) ->RegisterTicketConfig
        {
            RegisterTicketConfig
            {
                version:version,
                json:json,
                length:length
            }
        }

        //convert request msg
        pub fn work(& mut self)->String
        {
            let str_version=parse_num(self.version);
            let str_length=parse_num(self.length);
            let mut str_json=String::new();
            str_json.push_str(&self.json);
            let str_request=[str_version,str_length,str_json].concat();
            return str_request;
        }
    }
}

