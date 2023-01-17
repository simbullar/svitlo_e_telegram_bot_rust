mod token_gitignore; //delete this
//mod ping;
//mod sheets;
mod telegramus;
//use crate::ping::ping;
use std::{thread, time};
use urlencoding::encode;
use std::future::Future;

/*fn fill() -> impl Future<Output = ()>{
    sheets::fill;
}*/

fn main(){
    let mut id = "";
    let text_on = encode("Light is on");
    let text_off = encode("Light is off");
    let mut token = "";
    let mut ip = "";
    let mut status = "";
    let mut last_status= "";
    token_gitignore::change(&mut token); // delete this
    token_gitignore::change2(&mut id); // delete this
    token_gitignore::change3(&mut ip); // delete this
    let mut url_on = format!(r"https://api.telegram.org/bot{token}/sendMessage?chat_id={id}\&parse_mode=HTML&text={text_on}");
    let mut url_off = format!(r"https://api.telegram.org/bot{token}/sendMessage?chat_id={id}\&parse_mode=HTML&text={text_off}");
    println!("{}", url_on);
    telegramus::send(&mut url_on);

    /*loop{
        ping(&mut ip, &mut status, &mut last_status, &url_on, &url_off);
        thread::sleep(time::Duration::from_secs(60));
    }*/
}

