use seed::{prelude::*, *};
use web_sys::*;

pub fn is_full_screen() -> bool{
    document().fullscreen_element().is_some()
}
struct Model {
    pub video: ElRef<HtmlVideoElement>,
    pub playing: bool,
    pub video_container: ElRef<Element>,
    pub percentage_watched: f64,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            video: ElRef::default(),
            playing: false,
            video_container: ElRef::default(),
            // video_timeline: ElRef::default(),
            percentage_watched: 0.,
        }
    }
}

#[derive(Clone)]
enum Msg {
    SetTime(web_sys::PointerEvent),
    Fullscreen,
    FullscreenChanged,
    Play,
    Pause,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {


    match msg {
        Msg::SetTime(event) => {
            // let el = model.video.get().expect("No video el");
            // let total_duration = el.duration();
            // let x = event.client_x();
            // let y = event.client_y();
            // let video_timeline = model.video_timeline.get().unwrap();
            // let p = video_timeline.create_svg_point();
            // p.set_x(x as f32);
            // p.set_y(y as f32);
            // let ctm = video_timeline.get_screen_ctm().unwrap().inverse().unwrap();
            // let k = p.matrix_transform(&ctm);
            // model.percentage_watched = (k.x() as f64 - 5.) * 1.111 / 100.;
            // el.set_current_time(model.percentage_watched * total_duration);
        }
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
    }
}

fn view(model: &Model) -> impl View<Msg> {
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
        style! {
                St::Position => "absolute",
                St::Top => "0%",
                St::Bottom => "0%",
                St::Right => "190px",
                St::Left => "45px",
                St::BackgroundColor => "transparent",
                },
        div![
        // Video timeline
            style! {
                St::Position => "absolute",
                St::Top => "45%",
                St::Bottom => "45%",
                St::Right => "0%",
                St::Left => "0%",
                St::BorderRadius => "10px",
                St::BackgroundColor => "white",
                },
        ],
        div![
        // Video fill timeline
            style! {
                St::Position => "absolute",
                St::Top => "45%",
                St::Bottom => "45%",
                St::Right => "10%", // from 0% to 100% = progress bar
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
                St::Top => "13.75px",
                // St::Bottom => "45%",
                St::Right => "10%", // from 0% to 100% = progress bar
                // St::Left => "0%",
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
        style! {St::Width => "100%", St::Height => "100%", St::Overflow => "hidden", St::Margin => "auto"},
        video![
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
                St::Display => "flex";
                St::JustifyContent => "space-between";
                St::Height => "40px";
                St::BackgroundColor => "rgba(26,26,26,1.0)";
                St::Position => "relative";
                St::Top => if (is_full_screen()) {"-40px"} else{ "-40px" };
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



fn my_window_events(model: &Model) -> Vec<EventHandler<Msg>> {

    let mut result = Vec::new();
    result.push(simple_ev("onfullscreenchange", Msg::FullscreenChanged));
    result
}

