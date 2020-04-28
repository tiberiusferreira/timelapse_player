use seed::{prelude::*, *};
use web_sys::*;

struct Model {
    pub video: ElRef<HtmlVideoElement>,
    pub video_container: ElRef<Element>,
    // pub video_timeline: ElRef<SvgsvgElement>,
    pub percentage_watched: f64,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            video: ElRef::default(),
            video_container: ElRef::default(),
            // video_timeline: ElRef::default(),
            percentage_watched: 0.,
        }
    }
}

#[derive(Clone)]
enum Msg {
    SetTime(web_sys::PointerEvent),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
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
    }
}

fn view(model: &Model) -> impl View<Msg> {
    // let mut lines = vec![];
    // for i in 0..=300 {
    //     let stroke_width = 0.125;
    //     let x_pos = (i as f32 / 3.0) + stroke_width;
    //     let perc_bar = i as f64 / 300.;
    //     let color = if model.percentage_watched > perc_bar {
    //         "stroke:red; stroke-width:0.125"
    //     } else {
    //         "stroke:gray; stroke-width:0.125"
    //     };
    //     lines.push(line_![attrs! {
    //         At::X1 => (x_pos).to_string(),
    //         At::Y1 => "0",
    //         At::X2 => (x_pos).to_string(),
    //         At::Y2 => "100",
    //         At::Style => color,
    //     },]);
    // }
    let video_timeline = div![
        style! {
        St::Width => "100%",
        St::Height => "100%",
        St::BackgroundColor => "black",
        St::Position => "relative";},
        svg![
        // Play button
        style! {
            St::Position => "absolute",
            St::Top => "7px",
            St::Left => "10px",
            St::Height => "26px",
            St::Width => "26px",
        },
        attrs![At::ViewBox => "0 0 26 26", At::Fill => "white"],
        polygon![attrs![At::Points => "9.33 6.69 9.33 19.39 19.3 13.04 9.33 6.69"]],
        path![attrs![At::D => "M26,13A13,13,0,1,1,13,0,13,13,0,0,1,26,13ZM13,2.18A10.89,10.89,0,1,0,23.84,13.06,10.89,10.89,0,0,0,13,2.18Z"]]
        ],
        div![
        style! {
                St::Position => "absolute",
                St::Top => "0%",
                St::Bottom => "0%",
                St::Right => "50px",
                St::Left => "50px",
                St::BackgroundColor => "black",
                },
        div![
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
            style! {
                St::Position => "absolute",
                St::Top => "45%",
                St::Bottom => "45%",
                St::Right => "10%", // from 0% to 100% = progress bar
                St::Left => "0%",
                St::BorderRadius => "10px",
                St::BackgroundColor => "red",
                },
        ],],
        svg![
        // Full screen button
         style! {
            St::Position => "absolute",
            St::Top => "10px",
            St::Right => "10px",
            St::Height => "20px",
            St::Width => "20px",
        },
        attrs![At::ViewBox => "0 0 24 24", At::Fill => "white"],
        path![attrs![At::D => "M21.414 5.414l2.586 2.586v-8h-8l2.586 2.586-2.414 2.414h-8.344l-2.414-2.414 2.586-2.586h-8v8l2.586-2.586 2.414 2.414v8.344l-2.414 2.414-2.586-2.586v8h8l-2.586-2.586 2.414-2.414h8.344l2.414 2.414-2.586 2.586h8v-8l-2.586 2.586-2.414-2.414v-8.344z"]]
        ],
    ];
    //
    div![
        el_ref(&model.video_container),
        style! {St::Width => "100%", St::Height => "100%", St::Overflow => "hidden", St::Margin => "auto"},
        video![
            el_ref(&model.video),
            attrs! {At::Controls => "", At::Width => "100%", At::Height => "90%"},
            style! {St::Display => "block", St::Margin => "auto"},
            source![
                attrs! {At::Src => "http://192.168.15.29:8001/stream", At::Type => "video/mp4"}
            ],
        ],
        div![
            style! {
                St::Display => "flex";
                St::JustifyContent => "space-between";
                St::Height => "40px";
                St::BackgroundColor => "rgba(26,26,26,.8)";
                St::Position => "relative";
                St::Top => "-80px";
            },
            attrs! {At::Id => "video_buttons"},
            video_timeline,
            // button![
            //     attrs! {At::Class => "btn btn-primary bnt-sm"},
            //     style! {St::LineHeight => "70%"; St::Border => "1px"; St::BorderRadius => "5px"; St::Padding => "0px"; St::Width => "9%"; St::Height => "70%";St::Margin => "auto"},
            //     "5s"
            // ],
            // button![
            //     attrs! {At::Class => "btn btn-primary bnt-sm"},
            //     style! {St::LineHeight => "70%"; St::Border => "1px"; St::BorderRadius => "5px"; St::Padding => "0px"; St::Width => "9%"; St::Height => "70%";St::Margin => "auto"},
            //     "10s"
            // ],
            // button![
            //     attrs! {At::Class => "btn btn-primary bnt-sm"},
            //     style! {St::LineHeight => "70%"; St::Border => "1px"; St::BorderRadius => "5px"; St::Padding => "0px"; St::Width => "9%"; St::Height => "70%";St::Margin => "auto"},
            //     "20s"
            // ],
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
