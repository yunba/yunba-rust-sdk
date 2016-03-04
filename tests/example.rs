extern crate yunba_rust_sdk;

use yunba_rust_sdk::register_ticket_mod;
use yunba_rust_sdk::tcp_server_mod;
use yunba_rust_sdk::json_parser_mod;
use yunba_rust_sdk::mqtt_mod;
use std::net::Ipv4Addr;

fn register() ->(String,String,String)
{
    //
    //register module.
    //send register request to server,
    //and get response.
    //
    //register config:
    //register_ip:182,92,105,230 ,register_port:9944
    //
    let register_ip=Ipv4Addr::new(182,92,105,230);   //ip address
    let register_port=9944;
    let register_version=1;
    let register_json="{\"a\": \"5335292c44deccab56399e4f\", \"p\":2}".to_string();
    let register_length=register_json.len();

    //send request and get response
    let mut register=register_ticket_mod::register_ticket::RegisterTicketConfig::new(register_version,register_json,register_length); //send request to register server
    let register_request=register.work();
    println!("The register request :{}{}","\n",register_request);
    let mut register_tcp=tcp_server_mod::tcp::TcpServer::new(register_ip,register_port,register_request);
    register_tcp.send_request();
    register_tcp.get_response();
    let register_response=register_tcp.response; //get server response
    println!("The register response :{}{}","\n",register_response);

    //
    // parse json by json_parser
    //
    let mut register_json=json_parser_mod::json::JsonParser::new(register_response);
    let client_id=register_json.parser("c".to_string());
    println!("The register client_id :{}",client_id);
    let pass_word=register_json.parser("p".to_string());
    println!("The register pass_word :{}",pass_word);
    let user_name=register_json.parser("u".to_string());
    println!("The register user_name :{}",user_name);
    println!("{}","\n");

    return (client_id,pass_word,user_name);
}

fn ticket() -> String
{

    //
    //ticket module
    //sned ticket request to server,
    //and get response.
    //
    //ticket server config:
    //ticket_ip:123,57,32,238 ticket port:9977
    //
    let ticket_ip=Ipv4Addr::new(123,57,32,238);
    let ticket_port=9977;
    let ticket_version=1;
    let ticket_json="{\"a\": \"5335292c44deccab56399e4f\"}".to_string();
    let ticket_length=ticket_json.len();

    //
    //send request and get response
    //
    let mut ticket=register_ticket_mod::register_ticket::RegisterTicketConfig::new(ticket_version,ticket_json,ticket_length);
    let ticket_request=ticket.work();
    println!("The ticket request :{}{}","\n",ticket_request);
    let mut ticket_tcp=tcp_server_mod::tcp::TcpServer::new(ticket_ip,ticket_port,ticket_request);
    ticket_tcp.send_request();
    ticket_tcp.get_response();
    let ticket_response=ticket_tcp.response;
    println!("The ticket response :{}{}","\n",ticket_response);

    //
    //parse json by json_parser
    //
    let mut ticket_json=json_parser_mod::json::JsonParser::new(ticket_response);
    let client_config=ticket_json.parser("c".to_string());
    println!("The ticket tcp config :{}{}","\n",client_config);

    return client_config;
}

#[test]
fn mqtt()
{
    let (client_id,user_name,pass_word)=register();//get register information
    let client_config=ticket();//get ticket information
    //
    //mqtt server
    //
    let mut mqtt_client=mqtt_mod::mqtt_client::MqttConfig::new(client_id,user_name,pass_word,client_config,"MQIsdp".to_string());//mqtt_client.connect_to_broker();
    //mqtt_client.pub_get_alias_ack("hello".to_string(),"yunba".to_string());
    mqtt_client.pub_set_alias("sun".to_string());
    mqtt_client.pub_get_alias();
    //mqtt_client.pub_get_alias(",yaliget".to_string(),"".to_string());
    //mqtt_client.subscribe();
}
