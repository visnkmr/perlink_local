#![windows_subsystem = "windows"]
#[allow(warnings)]
use std::{env,rc, process::{self, ExitCode}};
use opentelemetry::{trace::{TraceError, Tracer, TraceContextExt, FutureExt, SpanKind, Span, get_active_span}, sdk::{trace::Config, Resource, propagation::TraceContextPropagator}, KeyValue, global, Key, Context};
use tracing::{info, span, log::warn, trace};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, fmt, util::SubscriberInitExt};
use window_titles::{Connection, ConnectionTrait};
use arboard::Clipboard;
use indexmap::{IndexMap};
extern crate linkify;
// mod log;
use linkify::{LinkFinder, LinkKind};
// use std::option::Option;
use fltk::{
    enums::{Color, FrameType, Event, CallbackTrigger},
    app::MouseButton,
    app::{App,*},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{TextBuffer, TextDisplay},
    window::Window,
    button::{Button,CheckButton},
   input::Input,
    prelude::*, frame::Frame,
};

use serde::{Deserialize, Serialize};
use std::{process::{Command,Stdio}, error::Error, time::Duration, thread};
// use execute::{Execute, command};

use isahc::prelude::*;
// extern crate preferences;
use std::collections::HashMap;
// use abserde::*;

use std::fs::create_dir_all;
// const APP_INFO: AppInfo = AppInfo{name: "Perlink", author: "visnk"};
const appname: &str = "perlink";
fn eurl(t: String) -> Result<String,()> {
    // return Ok("try".to_string());
    println!("get {} val----->{}","expanding",t);
    let mut response = isahc::get(
        format!("{}{}",prefstore::getcustom(appname, "website.su", "https://unshorten.me/s/".to_string()),t).as_str()
    ).map_err(|op|{
        eprintln!("Could not get expanded url. error:{}",op)
    }).unwrap();
    // println!("get {} val----->{}","expanded url",response.text()?);

    // Print some basic info about the response to standard output.
    // println!("Status: {}", response.status());
    // println!("Headers: {:#?}", response.headers());

    // Read the response body as text into a string and print it.
   
    return Ok(response.text().unwrap());
}
#[derive(Serialize, Deserialize, Default, Debug)]
struct MyConfig {
    shortenusing: String,
	// window_width: usize,
	// window_height: usize,
	// window_x: usize,
	// window_y: usize,
	// theme: String,
    #[serde(with = "indexmap::serde_seq")]
	user_data: IndexMap<String, String>,
}

// #[derive(Default)]
// struct vars{
//     jas: String,
// }

// const ChosenBrowser: &str = "dbrowser";
// const Notimes: &str = "ntimes";
// const Isenb: &str = "isenb";
// const PREFERENCES_KEY: &str = "prefs";
fn appendfile(browsername:String,browsercommand:String){
    prefstore::savepreference(appname, browsername,browsercommand);
    }
fn reinit(){
    // my_abserde.delete().expect("");    
    // let mut pref = IndexMap::<String,String>::new();
    let mut browsers;
    let browsers_names;
    // let mut browsers = ["V:\\Firefox\\firefox.exe","chromium","waterfox","vivaldi-stable","firefox-dev","firefox-beta"];
    #[cfg(not(target_os = "macos"))]{
        browsers = ["firefox","firefox","chromium","waterfox","vivaldi-stable","firefox-dev","firefox-beta"];
        browsers_names = ["firefox private window","firefox","chromium","waterfox","vivaldi stable","firefox dev","firefox beta"];
    }
    #[cfg(target_os = "macos")]{
        browsers = ["open -a Firefox --args --private-window","open -a Firefox --args","open -a Safari --args"];
        browsers_names = ["firefox private","firefox","safari"];
    }
    // #[cfg(not(target_os = "linux"))]{

    // }
    // setup();
    let mut i=0;
    for br in browsers{
        prefstore::savepreference(appname, br,browsers_names.get(i).unwrap().to_string());
        i+=1;
    }
    prefstore::savecustom(appname,"website.su", "https://unshorten.me/s/".to_string());
            
}

// }

pub fn link_finder_str(input: &str) -> Vec<String> {
    let mut links_str = Vec::new();
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let links: Vec<_> = finder.links(input).collect();

    for link in links.iter() {
        links_str.push(link.as_str().to_string());
    }
    links_str
}

use dotenv::dotenv;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>>  {
    dotenv().ok();
    // construct a subscriber that prints formatted traces to stdout
    // let subscriber = 
    // tracing_subscriber::FmtSubscriber::new();
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("perlink_main")
        .install_simple()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(opentelemetry)
        // Continue logging to stdout
        .with(fmt::Layer::default())
        .try_init()?;
    span!(tracing::Level::INFO, "init_started")
        .in_scope(||{
    
    let root = span!(tracing::Level::INFO, "init_setup", work_units = 2);
    info!("setup_crashreporting");
    let ac_key = env::var("APPCENTER_KEY").unwrap();
    app_center::start!(ac_key);
    human_panic::setup_panic!(human_panic::Metadata {
        version: env!("CARGO_PKG_VERSION").into(),
        name: env!("CARGO_PKG_NAME").into(),
        authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        path_to_save_log_to: prefstore::prefstore_directory(&appname.to_string()).unwrap(),
    });
    // let my_abserde = Abserde {
    //     app: appname.to_string(),
    //     location: Location::Auto,
    //     format: Format::Toml,
    // };
    info!("check_for_init_args");

    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        
        Some(val) => match val {
            val => {
                println!("{}----------->",val);

                if val == "reinit"{
                    // let mut initspan=global::tracer("perlink").start("initconfig");
                    info!("reinit");
                    println!("Reinitilizing config file.");
                    reinit();
                    // initspan.end();
                    process::exit(0);

                }if val == "add"{
                    info!("add_browser");
                    println!("Added new browser.");
                    appendfile(args.get(2).unwrap().to_string(),args.get(3).unwrap().to_string());
                    process::exit(0);

                }
                if val == "clear"{
                    info!("clear_browser_list");
                    println!("Cleared browser list.");
                    prefstore::clearall(appname,"txt");
                    process::exit(0);

                }
            }
            _ =>{
                
            },
            // Message::Stop => rlist(),
        },
        None => {
            
        },
    }
    
    
    let mut WIDGET_PADDING: i32 = 20;
    let mut WIDGET_WIDTH: i32 = 420;
    let mut WIDGET_HEIGHT: i32 = 400;  
    let args: Vec<String> = env::args().collect();
    let mut expandedurl = "".to_string();
    let mut ourl = "".to_string();
    // let mut sourl = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    // let mut sourl= vars{jas:"".to_string()};
    
    let mut strtoshow="";
    

    // let mut ourl = args.get(1).unwrap().to_string() ;
    // expandedurl = sk;
    // let (s, r) = fltk::app::channel();
    drop(root);
    let root = span!(tracing::Level::INFO, "loading_ui", work_units = 2);

            let mut app = App::default();
            
            let mut win = Window::default().with_size(WIDGET_WIDTH, WIDGET_HEIGHT).with_label("Choose browser");
            win.handle(move |f, ev|{
                // println!("{}----->{}",ev,fltk::app::event_text());
             match ev {
                fltk::enums::Event::KeyDown => {
                     if fltk::app::event_key() == fltk::enums::Key::from_char('f') {
                        // win.fullscreen(!win.fullscreen_active());
                    } else if fltk::app::event_key() == fltk::enums::Key::from_char('q') {
                        fltk::app::quit();
                    };
        
                    true
                }
                ,
                 _ => {
                     false
                 }
             }
});
let (s, r) = fltk::app::channel();

           
            let mut vpack=fltk::group::Pack::new(WIDGET_PADDING,
                WIDGET_PADDING,
                WIDGET_WIDTH - 40,
                WIDGET_HEIGHT - 40,"");
                win.resizable(&vpack);
              
                let mut framet = fltk::frame::Frame::default()
                .with_size(800,60)
                // .center_of(&win)
                .with_label("Loading");
              
            framet.set_label_size(12);
            let cfu = span!(tracing::Level::INFO, "get_url", work_units = 2);
            match args.get(1) {
                Some(val) => match val {

                    val => {
                        info!("Found_url_in_args");

                        expandedurl=val.to_string();
                        ourl=val.to_string();
                        setframe(&mut framet,&val);
                        // rt.set_label("");
                    }
                    _ =>{
                        info!("invalid_args");

                        expandedurl=" ".to_string();
                        ourl=" ".to_string();
                        setframe(&mut framet,&"invalid url".to_string());
                    },
                    // Message::Stop => rlist(),
                },
                None => {
                let cfu = span!(tracing::Level::INFO, "no_url_in_args", work_units = 2);

                    expandedurl=" ".to_string();
                    // let k=vars{jas:"".to_string()};
                    ourl=" ".to_string();
                    println!("here");
                    info!("Checking_in_window_titles");

                    let connection = Connection::new().unwrap();
                    // let mut pref = HashMap::<String,String>::new();
                    // let mut lks = vec!["", "New York"];
                    // let mut links: Vec<_>=;
                    for i in connection.window_titles().unwrap(){
                        // println!("{}",i.to_lowercase());
                        for kj in link_finder_str(&i){
                            info!("found_window");
                            let ss: String = kj.chars().skip(0).take(40).collect();
                            let mut b = Button::default()
                                    .with_size(70, 20)
                                    .with_label(&ss)
                                    // .with_align(Align::Left | Align::Inside)
                                    ;
                                    b.set_tooltip(&kj);
                                    b.emit(s.clone(),kj);
                                b.set_down_frame(FrameType::FlatBox);
                                b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
                                b.clear_visible_focus();
                             
                                b.set_frame(FrameType::FlatBox);
           
                        }
                    }

                    info!("Checking_in_clipboard");

                    let mut clipboard = Clipboard::new().unwrap();
                    match clipboard.get_text() {
                    Ok(sk) => { 
                        for kj in link_finder_str(&sk){
                    info!("found_clip");

                            let ss: String = kj.chars().skip(0).take(40).collect();
                            let mut b = Button::default()
                                .with_size(70, 20)
                                .with_label(&ss);
                            b.emit(s.clone(),kj.to_string());
                            b.set_tooltip(&kj);
                            b.set_down_frame(FrameType::FlatBox);
                            b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
                            b.clear_visible_focus();
                            b.set_frame(FrameType::FlatBox);
                        // println!("{}",kj);
                        }
                        
                        // fltk::dialog::message(90, 90, &sk);{
                            // let mut res = std::process::Command::new(format!("/home/roger/Downloads/waterfox/waterfox {}",sk)).output();
                        // }
                        
                        // ... use sk ...
                    },
                    Err(e) => {
                    info!("error_fetch_clipboard");

                        println!("Error Clipboard");
                        // setframe(&mut framet,"Error");
                        // ... sk is not available, and e explains why ...
                    },
                }
                drop(cfu);
                                       
                }
                    
            ,
            }
            
            drop(cfu);
            // println!("{}",ourl);
                fltk::frame::Frame::default().with_size(20, 10);
           
            let mut ttb=fltk::group::Pack::default().with_size(
                10,
                40) ;
                fltk::frame::Frame::default().with_size(20, 30);
            
            let mut eub = Button::default().with_size(150,30);
            eub.set_label("expand url");
            eub.emit(s.clone(),"expandurl".to_string());
            
            fltk::frame::Frame::default().with_size(20, 10);
            // let mut bframe1 = fltk::frame::Frame::default().with_size(300, 60);
            let mut b11 = Button::default().with_size(150,30);
            b11.set_label("All browsers");
            // b1.emit(s, "refresh".to_string());
            // let mut hpack=hpack.clone();
            b11.emit(s.clone(),"all".to_string());
            

            ttb.end();
            ttb.set_type(fltk::group::PackType::Horizontal);
            fltk::frame::Frame::default().with_size(10, 10);
            let mut ttb=fltk::group::Pack::default().with_size(
                10,
                40) ;
                
                
                fltk::frame::Frame::default().with_size(20, 30);
            
                let mut svw = Button::default().with_size(150,30);
                svw.set_label("share via web");
                svw.emit(s.clone(),"svw".to_string());
                
            
            fltk::frame::Frame::default().with_size(20, 10);
            // let mut bframe1 = fltk::frame::Frame::default().with_size(300, 60);
            let mut svc = Button::default().with_size(150,30);
                svc.set_label("copy to clipboard");
                svc.emit(s.clone(),"svc".to_string());
    

            ttb.end();
            ttb.set_type(fltk::group::PackType::Horizontal);
            fltk::frame::Frame::default().with_size(20, 30);
            let mut hpack=fltk::group::Pack::default().with_size(250,40) .center_of(&win);
                // let i=0;

                // browsers=browsers.clone();
                let mut i=0;
                // let mut bl:PreferencesMap<String> = setup();
                if(prefstore::getall(appname).is_empty()){
                    reinit();
                }
                
                for (k,v) in prefstore::getall(appname) {
                    let expandedurl=expandedurl.clone();
                    fltk::frame::Frame::default().with_size(20, 10);
                    let k: String = k.chars().skip(0).take(10).collect();
                    // let cc = k.chars().count();
                    // let sz=cc*9;
                    // let mut b1 = Button::default().with_size(sz.try_into().unwrap(),60);
                    let mut b1 = Button::default().with_size(90,60);
                    
                    b1.set_label(&format!("{}",k));
                    b1.emit(s.clone(),v);
                    
                    i+=1;
                    if(i%3 ==0){
                        // println!("i value--------->{}",i);
                        hpack.end();
                    hpack.set_type(fltk::group::PackType::Horizontal);
                    fltk::frame::Frame::default().with_size(20, 10);
                    hpack=fltk::group::Pack::default().with_size(250,40) .center_of(&win);
                    }
                }
                // let browsers = "";

            hpack.end();
            hpack.set_type(fltk::group::PackType::Horizontal);
            win.make_resizable(true);
            // win.resizable(&vpack);

            vpack.end();    
            vpack.set_type(fltk::group::PackType::Vertical);
            
            win.show_with_env_args();

            win.end();
            win.show();
            drop(root);
            span!(tracing::Level::INFO, "waiting_for_input")
        .in_scope(|| {
            info!("waiting for input");
            // let mut frame1 =frame.clone();
            // get_active_span(|span|async{
                while app.wait() {
                // setframe(&mut frame, "");
                // frame=frame.clone();
                match r.recv() {
                    
                    Some(val) => 
                    match val {
                        val => {
                            // get_active_span(|span| {
                            //     span.add_event("An event!".to_string(), vec![KeyValue::new("happened", true)]);
                            // });
                            
                            // if(val == "frominput"){
                            //             ourl=url.value();
                            //     }
                            // let mut str=val;
                            if(val.contains("//")){
                                info!("expanded_url");
                                // let k= format!("{}",val);
                                // frame.set_label(&k);
                                setframe(&mut framet, &val);
                                // println!("//------------->");

                                // println!("{}",format!("{}",val));
                            ourl=format!("{}",val);
                            expandedurl=val;
                            // rt.set_label("title");
                            // frame.set_label("");
                            // setframe(&mut frame,"");
                            
                            true;
                            }
                            else if val == "expandurl"{
                                info!("expand_url");
                                match eurl(ourl.clone()) {
                                    Ok(sk) => { 
                                        if(sk.to_lowercase().contains("invalid")){
                                            setframe(&mut framet,args.get(1).unwrap());
                                            // rt.set_label("");
                                        }
                                        else{
                                            setframe(&mut framet, &sk);
                                        }
                                        
                                        // fltk::dialog::message(90, 90, &sk);{
                                            // let mut res = std::process::Command::new(format!("/home/roger/Downloads/waterfox/waterfox {}",sk)).output();
                                        // }
                                        
                                        // ... use sk ...
                                    },
                                    Err(e) => {
                                        setframe(&mut framet,"Error");
                                        // ... sk is not available, and e explains why ...
                                    },
                                }
                            }
                            else if(val == "all"){
                                // println!("all------------->");
                                // span.add_event("opening".to_string(), vec![]);
                                // if ourl==" "{
                                //     ourl=url.value(); 
                                info!("opening_in_all_browsers");
                                let root = span!(tracing::Level::INFO, "opening_in_all", work_units = 2);
                                //  }
                                if(prefstore::getall(appname).is_empty()){
                                    reinit();
                                }
                                let(hmap)=prefstore::getall(appname);
                                
    ;
                            
                                // let cx = Context::current();
                                // let span = cx.span();
                                // span.add_event("Opening in all browsers".to_string(), vec![]);
                                // span.add_event("openinall".to_string(), vec![]);
                                for (_,v) in hmap{

                                        open(&v,&ourl);
                                }
                                drop(root);
                                true;
                            }
                            else if(val == "svc"){
                                info!("fromclipboard");
                                let mut clipboard = Clipboard::new().unwrap();
                                // println!("{}",&ourl);
                                #[cfg(target_os = "linux")]{
                                    clipboard.set().wait().text(&ourl).unwrap();
                                }
                                #[cfg(not(target_os = "linux"))]{
                                    clipboard.set_text(&ourl).unwrap();
                                }
                                // clipboard.set_text("abc".to_string()).unwrap();
                                // println!("{}",clipboard.get_text().unwrap());
                            }else if(val == "svw"){
                                // ada
                            }
                            // else 
                            else{
                                let root = span!(tracing::Level::INFO, "clicked", work_units = 2);
                                info!("clicked_{val}");
                                // if ourl==" "{
                                //     ourl=url.value(); 
                                //  }
                                 
                                // println!("{}------------->r{}r",val,expandedurl);
    // let tracer = global::tracer("opentracer");

                // span.add_event(val.to_string(), vec![]);

                
                            // let tracer = global::tracer("init");
                            
                            
                            // let cx = Context::current();
                            // let span = cx.span();
                            // span.add_event("opening in browser".to_string(), vec![]);
                                open(&val,&expandedurl);
                                // .with_context(cx).await;
                                // println!("opening----->{}",expandedurl);
                                drop(root);
                                
                                fltk::app::quit();
                                                true;
                            }
                            
                            // frame.set_label(&val);
                            
                        },
                        // Message::Stop => rlist(),
                    },
                    None => ({
                        // println!("stop");
                    })
                }
                
                // let frame=win.frame.clone();
                // frame.set_label("&val");
            }
        });
            warn!("Exiting");
        });
            // .with_context(cx);
            // });
        // global::shutdown_tracer_provider();
    process::exit(0);
            // app.run().unwrap();    
            Ok(())
}
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
const DAEMONIZE_ARG: &str = "__internal_daemonize";

fn setframe(f:&mut Frame,s: &str){
    let ss: String = s.chars().skip(0).take(40).collect();
    f.set_label(&ss);
}
// async 
fn open(v: &String,ourl: &String)->Result<(),()>{
    let root = span!(tracing::Level::INFO, "opening_browser", work_units = 2);
   

    let strings:Vec<String> = v.split_whitespace().map(str::to_string).collect();
    let mut res = Command::new(format!("{}",strings[0]));
    let slice = &strings[1..strings.len()];

    for k in slice{ 
        res.arg(k);
    }

    let tte=res.arg(format!("{}",ourl))
                        .spawn()
                        .expect("failed to execute process");
    eprintln!("{:?}",tte);
    drop(root);

    Ok(())
}