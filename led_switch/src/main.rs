fn main(){
    rainmaker::rainmaker_init();
    rainmaker::rainmaker_say_hello();
    rainmaker::http_server().unwrap();
}
