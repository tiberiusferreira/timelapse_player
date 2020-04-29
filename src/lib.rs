use seed::{prelude::*, *};
use web_sys::*;

pub fn is_full_screen() -> bool{
    document().fullscreen_element().is_some()
}
struct Model {
    pub video: ElRef<HtmlVideoElement>,
    pub progress_bar: ElRef<Element>,
    pub playing: bool,
    pub video_container: ElRef<Element>,
    pub percentage_watched: f64,
    pub controls_opacity: f64,
    pub last_wake: f64,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            video: ElRef::default(),
            progress_bar: Default::default(),
            playing: false,
            video_container: ElRef::default(),
            percentage_watched: 0.,
            controls_opacity: 0.0,
            last_wake: js_sys::Date::now()
        }
    }
}

#[derive(Clone)]
enum Msg {
    Fullscreen,
    FullscreenChanged,
    TooglePlayPause,
    Play,
    Pause,
    SeekTo(web_sys::PointerEvent),
    AddSec(f64),
    WakeControls,
    SleepControls,
    Nothing,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {


    match msg {
        Msg::Fullscreen => {
            if is_full_screen(){
                document().exit_fullscreen();
            }else{
                let video_as_el = model.video_container.get().unwrap();
                video_as_el.request_fullscreen().unwrap();
            }
        }
        Msg::Play => {
            model.video.get().unwrap().play();
            model.playing = true;
        }
        Msg::Pause => {
            model.video.get().unwrap().pause();
            model.playing = false;
        }
        Msg::FullscreenChanged => {
            orders.render();
        }
        Msg::SeekTo(pointer_ev) => {
            let progress_bar_html = model.progress_bar.get().unwrap();
            let width = progress_bar_html.get_bounding_client_rect().width();
            let x = progress_bar_html.get_bounding_client_rect().x();
            let c_x = pointer_ev.client_x();
            let perc = (c_x as f64 -x)/width;
            let video = model.video.get().unwrap();
            let duration = video.duration();
            video.set_current_time(perc*duration);
            model.percentage_watched = perc;
        }
        Msg::AddSec(secs) => {
            let video = model.video.get().unwrap();
            let curr_time = video.current_time();
            video.set_current_time(curr_time+secs);
        }
        Msg::TooglePlayPause => {
            if model.playing{
                model.video.get().unwrap().pause();
            }else{
                model.video.get().unwrap().play();
            }
            model.playing = !model.playing;
        }
        Msg::Nothing => {}
        Msg::WakeControls => {
            model.controls_opacity = 1.;
            model.last_wake = js_sys::Date::now();
            orders.perform_cmd(start_sleep_controls_timer());
        },
        Msg::SleepControls => {
            if js_sys::Date::now()-model.last_wake > (1_500. - 200.){
                model.controls_opacity = 0.;
            }
        }
    }
}

async fn start_sleep_controls_timer() -> Result<Msg, Msg>{
    use gloo_timers::future::TimeoutFuture;
    TimeoutFuture::new(1_500).await;
    Ok(Msg::SleepControls)
}

fn view(model: &Model) -> impl View<Msg> {
    let watched_perc = format!("{}%", 100.*(1.-model.percentage_watched));
    let video_timeline = div![

        style! {
        St::Width => "100%",
        St::Height => "100%",
        St::BackgroundColor => "transparent",
        St::Position => "relative";},

        if model.playing{
            svg![
            // Pause button
            simple_ev(Ev::Click, Msg::Pause),
            style! {
                St::Position => "absolute",
                St::Top => "7.5px",
                St::Left => "10px",
                St::Height => "25px",
                St::Width => "25px",
            },
            attrs![At::ViewBox => "0 0 512 512", At::Fill => "white"],
            g![
                path![attrs![At::D => "M224,435.8V76.1c0-6.7-5.4-12.1-12.2-12.1h-71.6c-6.8,0-12.2,5.4-12.2,12.1v359.7c0,6.7,5.4,12.2,12.2,12.2h71.6   C218.6,448,224,442.6,224,435.8z"]],
                path![attrs![At::D => "M371.8,64h-71.6c-6.7,0-12.2,5.4-12.2,12.1v359.7c0,6.7,5.4,12.2,12.2,12.2h71.6c6.7,0,12.2-5.4,12.2-12.2V76.1   C384,69.4,378.6,64,371.8,64z"]]
            ],
            ]
        }else{
            svg![
            // Play button
            simple_ev(Ev::Click, Msg::Play),
            style! {
                St::Position => "absolute",
                St::Top => "2.5px",
                St::Left => "5px",
                St::Height => "35px",
                St::Width => "35px",
            },
            attrs![At::ViewBox => "0 0 26 26", At::Fill => "white"],
            polygon![attrs![At::Points => "9.33 6.69 9.33 19.39 19.3 13.04 9.33 6.69"]],
            ]
        },
        div![
        // Timeline container
        pointer_ev(Ev::PointerDown, Msg::SeekTo),
        style! {
                St::Position => "absolute",
                St::Top => "25%",
                St::Bottom => "25%",
                St::Right => "190px",
                St::Left => "45px",
                St::BackgroundColor => "transparent",
                },
        div![
        // Video timeline
        el_ref(&model.progress_bar),
        pointer_ev(Ev::PointerDown, Msg::SeekTo),
            style! {
                St::Position => "absolute",
                St::Top => "35%",
                St::Bottom => "35%",
                St::Right => "0%",
                St::Left => "0%",
                St::BorderRadius => "10px",
                St::BackgroundColor => "white",
                },
        ],
        div![
        // Video fill timeline
            pointer_ev(Ev::PointerDown, Msg::SeekTo),
            style! {
                St::Position => "absolute",
                St::Top => "35%",
                St::Bottom => "35%",
                St::Right => watched_perc, // from 0% to 100% = progress bar
                St::Left => "0%",
                St::BorderRadius => "10px",
                St::BackgroundColor => "red",
                },
        ],
        div![
        // Video fill ball
            style! {
                St::Position => "absolute",
                St::Width => "12.5px",
                St::Height => "12.5px",
                St::Top => "calc(12.5% + 1px)",
                // from 0% to 100% = progress bar, but reversed. Also need to account for the ball size
                St::Right => format!("calc({}% - 6.25px)", 100.*(1.-model.percentage_watched))
                St::BorderRadius => "50%",
                St::BackgroundColor => "red",
                },
        ],
        ],
        svg![
        // Full screen button
        simple_ev(Ev::Click, Msg::Fullscreen),
         style! {
            St::Position => "absolute",
            St::Top => "10px",
            St::Right => "155px",
            St::Height => "20px",
            St::Width => "20px",
        },
        attrs![At::ViewBox => "0 0 24 24", At::Fill => "white"],
        path![attrs![At::D => "M21.414 5.414l2.586 2.586v-8h-8l2.586 2.586-2.414 2.414h-8.344l-2.414-2.414 2.586-2.586h-8v8l2.586-2.586 2.414 2.414v8.344l-2.414 2.414-2.586-2.586v8h8l-2.586-2.586 2.414-2.414h8.344l2.414 2.414-2.586 2.586h8v-8l-2.586 2.586-2.414-2.414v-8.344z"]]
        ],

        button![
        simple_ev(Ev::Click, Msg::AddSec(-30.)),
        style! {
            // second right most button
            St::Position => "absolute",
            St::Top => "5px",
            St::Right => "115px",
            St::Height => "30px",
            St::Width => "30px",
            St::Padding => "0px",
            St::BorderRadius=> "5px",
            St::Border => "0px",
            St::Color => "white",
            St::BackgroundColor => "transparent",
        },
        "-30s"],

        button![
        simple_ev(Ev::Click, Msg::AddSec(-5.)),
        style! {
            // third right most button
            St::Position => "absolute",
            St::Top => "5px",
            St::Right => "80px",
            St::Height => "30px",
            St::Width => "30px",
            St::Padding => "0px",
            St::BorderRadius=> "5px",
            St::Border => "0px",
            St::Color => "white",
            St::BackgroundColor => "transparent",
        },
        "-5s"],

        button![
        simple_ev(Ev::Click, Msg::AddSec(5.)),
        style! {
            // second right most button
            St::Position => "absolute",
            St::Top => "5px",
            St::Right => "45px",
            St::Height => "30px",
            St::Width => "30px",
            St::Padding => "0px",
            St::BorderRadius=> "5px",
            St::Border => "0px",
            St::Color => "white",
            St::BackgroundColor => "transparent",
        },
        "+5s"],

        button![
        simple_ev(Ev::Click, Msg::AddSec(30.)),
        style! {
            // right most button
            St::Position => "absolute",
            St::Top => "5px",
            St::Right => "10px",
            St::Height => "30px",
            St::Width => "30px",
            St::Padding => "0px",
            St::BorderRadius=> "5px",
            St::Border => "0px",
            St::Color => "white",
            St::BackgroundColor => "transparent",
        },
        "+30s"],

    ];

    div![
        el_ref(&model.video_container),
        simple_ev(Ev::PointerDown, Msg::WakeControls),
        simple_ev(Ev::PointerMove, Msg::WakeControls),
        style! {St::Width => "100%", St::Height => "100%", St::Overflow => "hidden", St::Margin => "auto"},
        video![
            simple_ev(Ev::PointerDown, Msg::TooglePlayPause),
            el_ref(&model.video),
            attrs! {At::Width => "100%", At::Height => if is_full_screen() {"100%"} else { "90%" };},
            style! {St::Display => "block", St::Margin => "auto"},
            source![
                attrs! {At::Src => "http://192.168.15.29:8001/stream", At::Type => "video/mp4"}
            ],
        ],
        div![
            id!["video_controls"],
            style! {
                St::Opacity => model.controls_opacity;
                St::Display => "flex";
                St::JustifyContent => "space-between";
                St::Height => "40px";
                St::BackgroundColor => "rgba(26,26,26,1.0)";
                St::Position => "relative";
                St::Top => "-40px";
            },
            video_timeline,
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    use wasm_bindgen::JsCast;

    let app = App::builder(update, view)
        .window_events(my_window_events)
        .build_and_start();


    let cb = Closure::wrap(Box::new(move || {
        app.update(Msg::FullscreenChanged)
    }) as Box<dyn FnMut()>);

    seed::window().document().unwrap().set_onfullscreenchange(Some(cb.as_ref().unchecked_ref()));

    cb.forget();
}



fn my_window_events(_model: &Model) -> Vec<EventHandler<Msg>> {

    let mut result = Vec::new();
    result.push(keyboard_ev("keydown", |ev| {
        if ev.key() == " " {
            Msg::TooglePlayPause
        }else{
            Msg::Nothing
        }
    }));
    result
}

