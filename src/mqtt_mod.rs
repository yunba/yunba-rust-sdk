///
///mqtt module
///

extern crate mqtt;
extern crate regex;

pub mod mqtt_client
{
    use std::io::Write;
    use std::net::TcpStream;
    use std::io::Cursor;
    use std::str;

    use regex::Regex;

    use mqtt::{Encodable, Decodable, QualityOfService};
    use mqtt::packet::*;
    use mqtt::TopicName;
    use mqtt::control::variable_header::ConnectReturnCode;
    use mqtt::TopicFilter;

    pub struct MqttConfig
    {
        pub client_id:String,
        pub user_name:String,
        pub pass_word:String,
        pub ip_port:String,
        pub protocol_name:String

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
    impl MqttConfig
    {
        pub fn new(client_id:String,user_name:String,pass_word:String,ip_port:String,protocol_name:String) -> MqttConfig
        {
            let re=Regex::new("[0-9]{1,3}.[0-9]{1,3}.[0-9]{1,3}.[0-9]{1,3}:[0-9]{4}").unwrap();
            let result=re.find(&ip_port);
            let mut start=0;
            let mut end=0;
            match result
            {
                Some((x,y))=>
                {
                    start=x;
                    end=y;
                },
                None=>println!("regex match failed in mqtt_mod"),
            }
            let str_ip_port=&ip_port[start..end];
            MqttConfig
            {
                client_id:client_id,
                user_name:user_name,
                pass_word:pass_word,
                ip_port:str_ip_port.to_string(),
                protocol_name:protocol_name
            }
        }

        pub fn connect_to_broker(& mut self)
        {
            let ip_port:&str=&self.ip_port[..];
            let mut stream=TcpStream::connect(ip_port).unwrap();
            let mut client=ConnectPacket::new(self.protocol_name.to_owned(),self.client_id.to_owned());
            client.set_user_name(Some(self.user_name.to_owned()));
            client.set_password(Some(self.pass_word.to_owned()));
            client.set_clean_session(true);
            let mut buf=Vec::new();
            client.encode(&mut buf).unwrap();
            stream.write_all(&buf[..]).unwrap();

            let connack = ConnackPacket::decode(&mut stream).unwrap();
            println!("{}CONNACK {:?}{}","RECV: ",connack,"\n");

            if connack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
                panic!("{}Failed to connect to server, return code {:?}",
                       "RECV: ",connack.connect_return_code());
            }

            let response=VariablePacket::decode(&mut stream);
            println!("{}{:?}{}","RECV: ",response,"\n");

        }

        pub fn publish(&mut self,topic:String,content:String)
        {
            let ip_port:&str=&self.ip_port[..];
            let mut stream=TcpStream::connect(ip_port).unwrap();
            let mut client=ConnectPacket::new(self.protocol_name.to_owned(),self.client_id.to_owned());
            client.set_user_name(Some(self.user_name.to_owned()));
            client.set_password(Some(self.pass_word.to_owned()));
            client.set_clean_session(true);
            let mut buf=Vec::new();
            client.encode(&mut buf).unwrap();
            stream.write_all(&buf[..]).unwrap();

            let connack = ConnackPacket::decode(&mut stream).unwrap();
            println!("{}CONNACK {:?}{}","RECV: ",connack,"\n");

            if connack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
                panic!("{}Failed to connect to server, return code {:?}",
                       "RECV: ",connack.connect_return_code());
            }
            // Create a new Publish packet
            let vec_content=content.into_bytes();
            let packet = PublishPacket::new(TopicName::new(topic).unwrap(),
            PublishQoSWithPacketIdentifier::Level2(10),
            vec_content);

            // Encode
            let mut buf = Vec::new();
            packet.encode(&mut buf).unwrap();
            stream.write_all(&buf[..]).unwrap();
            println!("{}Publish Packet Encoded: {:?}{}","SENT: ",buf,"\n");

            // Decode it with known type
            let mut dec_buf = Cursor::new(&buf[..]);
            let decoded = PublishPacket::decode(&mut dec_buf).unwrap();
            println!("{}Publish Packet Decoded: {:?}{}","RECV: ",decoded,"\n");
            assert_eq!(packet, decoded);


            loop
            {
                let response = match VariablePacket::decode(&mut stream)
                {
                    Ok(pk) => pk,
                    Err(err) =>
                    {
                        println!("Error in receiving packet: {}{}",err,"\n");
                        break;
                    }
                };

                match &response
                {
                    &VariablePacket::PingreqPacket(..) =>
                    {
                        let pingresp = PingrespPacket::new();
                        println!("Sending Ping response {:?}{}", pingresp,"\n");
                        pingresp.encode(&mut stream).unwrap();
                    }

                    &VariablePacket::PublishPacket(ref publ) =>
                    {
                        let msg = match str::from_utf8(&publ.payload()[..])
                        {
                            Ok(msg) => msg,
                            Err(err) =>
                            {
                                println!("Failed to decode publish message {:?}{}", err,"\n");
                                break;
                            }
                        };
                        println!("PUBLISH ({}): {}{}", publ.topic_name(), msg,"\n");
                    }

                    _ => {}
                }
            }
    }
        pub fn pub_set_alias(&mut self,content:String)
        {
            MqttConfig::publish(self,",yali".to_string(),content);
        }

        pub fn pub_get_alias(&mut self)
        {
            MqttConfig::publish(self,",yaliget".to_string(),"".to_string());
        }

        pub fn pub_get_alias_ack(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,2,topic,content);
        }

        pub fn pub_get_topic_list(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,3,topic,content);
        }

        pub fn pub_get_topic_list_v2(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,13,topic,content);
        }

        pub fn pub_get_topic_list_ack(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,4,topic,content);
        }

        pub fn pub_get_topic_list_ack_v2(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,14,topic,content);
        }

        pub fn pub_get_aliaslist(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,5,topic,content);
        }

        pub fn pub_get_aliaslist_v2(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,15,topic,content);
        }

        pub fn pub_get_aliaslist_ack(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,6,topic,content);
        }

        pub fn pub_get_aliaslist_ack_v2(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,16,topic,content);
        }

        pub fn pub_new_publish(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,7,topic,content);
        }

        pub fn pub_new_puback(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,8,topic,content);
        }

        pub fn pub_get_status(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,9,topic,content);
        }

        pub fn pub_get_status_v2(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,19,topic,content);
        }

        pub fn pub_get_status_ack(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,10,topic,content);
        }

        pub fn pub_get_status_ack_v2(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,20,topic,content);
        }

        pub fn pub_recvack(&mut self,topic:String,content:String)
        {
            MqttConfig::publish_alias(self,11,topic,content);
        }

        pub fn publish_alias(&mut self,alias:usize,topic:String,content:String)
        {
            //connect to broker
            let ip_port:&str=&self.ip_port[..];
            let mut stream=TcpStream::connect(ip_port).unwrap();
            let mut client=ConnectPacket::new(self.protocol_name.to_owned(),self.client_id.to_owned());
            client.set_user_name(Some(self.user_name.to_owned()));
            client.set_password(Some(self.pass_word.to_owned()));
            client.set_clean_session(true);
            let mut buf=Vec::new();
            client.encode(&mut buf).unwrap();
            stream.write_all(&buf[..]).unwrap();

            let connack = ConnackPacket::decode(&mut stream).unwrap();
            println!("{}CONNACK {:?}{}","RECV: ",connack,"\n");

            if connack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
                panic!("{}Failed to connect to server, return code {:?}",
                       "RECV: ",connack.connect_return_code());
            }

            //match alias command
            let cmd_binary=match alias
            {
                1  => parse_num(alias),
                2  => parse_num(alias),
                3  => parse_num(alias),
                13 => parse_num(alias),
                4  => parse_num(alias),
                14 => parse_num(alias),
                5  => parse_num(alias),
                15 => parse_num(alias),
                6  => parse_num(alias),
                16 => parse_num(alias),
                7  => parse_num(alias),
                8  => parse_num(alias),
                9  => parse_num(alias),
                19 => parse_num(alias),
                10 => parse_num(alias),
                20 => parse_num(alias),
                _  => {
                    println!("Alias Command Error");
                    "".to_string()
                },
            };
            // Create a new PublishAlias packet
            let str_payload=[cmd_binary,content].concat();
            let mut vec_payload=str_payload.into_bytes();
            vec_payload.push(0);
            let packet = PublishAliasPacket::new(TopicName::new(topic).unwrap(),
            PublishAliasQoSWithPacketIdentifier::Level2(10),
            vec_payload);

            // Encode
            let mut buf = Vec::new();
            packet.encode(&mut buf).unwrap();
            stream.write_all(&buf[..]).unwrap();
            println!("{}PublishAlias Packet Encoded: {:?}{}", "SENT: ",buf,"\n");

            // Decode it with known type
            let mut dec_buf = Cursor::new(&buf[..]);
            let decoded = PublishAliasPacket::decode(&mut dec_buf).unwrap();
            println!("{}PublishAlias Packet Decoded: {:?}{}", "RECV: ",decoded,"\n");
            assert_eq!(packet, decoded);

            //receive response
            loop
            {
                let response = match VariablePacket::decode(&mut stream)
                {
                    Ok(pk) => pk,
                    Err(err) =>
                    {
                        println!("Error in receiving packet: {}{}",err,"\n");
                        break;
                    }
                };

                match &response
                {
                    &VariablePacket::PingreqPacket(..) =>
                    {
                        let pingresp = PingrespPacket::new();
                        println!("Sending Ping response {:?}{}", pingresp,"\n");
                        pingresp.encode(&mut stream).unwrap();
                    }

                    &VariablePacket::PublishPacket(ref publ) =>
                    {
                        let msg = match str::from_utf8(&publ.payload()[..])
                        {
                            Ok(msg) => msg,
                            Err(err) =>
                            {
                                println!("Failed to decode publish message {:?}{}", err,"\n");
                                break;
                            }
                        };
                        println!("PUBLISH ({}): {}{}", publ.topic_name(), msg,"\n");
                    }

                    _ => {}
                }
            }
        }

        pub fn subscribe(&mut self)
        {

            //connect to broker
            let ip_port:&str=&self.ip_port[..];
            let mut stream=TcpStream::connect(ip_port).unwrap();
            let mut client=ConnectPacket::new(self.protocol_name.to_owned(),self.client_id.to_owned());
            client.set_user_name(Some(self.user_name.to_owned()));
            client.set_password(Some(self.pass_word.to_owned()));
            client.set_clean_session(true);
            let mut buf=Vec::new();
            client.encode(&mut buf).unwrap();
            stream.write_all(&buf[..]).unwrap();

            let connack = ConnackPacket::decode(&mut stream).unwrap();
            println!("CONNACK {:?}{}",connack,"\n");

            if connack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
                panic!("Failed to connect to server, return code {:?}",
                       connack.connect_return_code());
            }
            let topic=TopicFilter::new("Hello yunba");

            //Create a new subscribes packet
            let channel_filters:Vec<(TopicFilter,QualityOfService)>=
                vec![(topic,QualityOfService::Level0)];
            let sub=SubscribePacket::new(10,channel_filters);

            //Encode
            let mut buf=Vec::new();
            sub.encode(&mut buf).unwrap();
            stream.write_all(&buf[..]).unwrap();
            println!("{}Subscribe Packet Encoded: {:?}{}","SENT: ",buf,"\n");

            //Decode it with known type
            let mut dec_buf=Cursor::new(&buf[..]);
            let decoded=SubscribePacket::decode(&mut dec_buf).unwrap();
            println!("{}Subscribe Packet Decoded: {:?}{}","RECV: ",decoded,"\n");
            assert_eq!(sub,decoded);


            let response=VariablePacket::decode(&mut stream);
            println!("{}{:?}{}","RECV: ",response,"\n");
        }

    }
}

