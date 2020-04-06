use seed::{prelude::*, *};
use web_sys::*;

struct Model {
    pub video: ElRef<HtmlVideoElement>,
    pub video_timeline: ElRef<SvgsvgElement>,
    pub percentage_watched: f64,
    pub moving_left_cursor: bool,
    pub cursor_left_pos: f64,
    pub moving_right_cursor: bool,
    pub cursor_right_pos: f64,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            video: ElRef::default(),
            video_timeline: ElRef::default(),
            percentage_watched: 0.,
            moving_left_cursor: false,
            cursor_left_pos: 5.,
            moving_right_cursor: false,
            cursor_right_pos: 90.,
        }
    }
}

#[derive(Clone)]
enum Msg {
    SetTime(web_sys::PointerEvent),
    SetTimeZoom(web_sys::PointerEvent),
    GrabbedLeft,
    GrabbedRight,
    PointerMoved(web_sys::PointerEvent),
    PointerUp(web_sys::PointerEvent),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetTime(event) => {
            let el = model.video.get().expect("No video el");
            let total_duration = el.duration();
            let x = event.client_x();
            let y = event.client_y();
            let video_timeline = model.video_timeline.get().unwrap();
            let p = video_timeline.create_svg_point();
            p.set_x(x as f32);
            p.set_y(y as f32);
            let ctm = video_timeline.get_screen_ctm().unwrap().inverse().unwrap();
            let k = p.matrix_transform(&ctm);
            model.percentage_watched = (k.x() as f64 - 5.) * 1.111 / 100.;
            el.set_current_time(model.percentage_watched * total_duration);
        }
        Msg::GrabbedLeft => {
            log!("Grabbed Left!");
            // let a = model.cursor_left_el.get().expect("No cursor left");
            // a.set_pointer_capture(event.pointer_id()).unwrap();
            model.moving_left_cursor = true;
        }
        Msg::PointerUp(_) => {
            log!("Released All!");
            model.moving_left_cursor = false;
            model.moving_right_cursor = false;
        }
        Msg::PointerMoved(event) => {
            log!("Moved!");
            if model.moving_left_cursor {
                let x = event.client_x();
                let y = event.client_y();
                let video_timeline = model.video_timeline.get().unwrap();
                let p = video_timeline.create_svg_point();
                p.set_x(x as f32);
                p.set_y(y as f32);
                let ctm = video_timeline.get_screen_ctm().unwrap().inverse().unwrap();
                let k = p.matrix_transform(&ctm);
                let x = k.x();
                model.cursor_left_pos = x as f64;
            }
            if model.moving_right_cursor {
                let x = event.client_x();
                let y = event.client_y();
                let video_timeline = model.video_timeline.get().unwrap();
                let p = video_timeline.create_svg_point();
                p.set_x(x as f32);
                p.set_y(y as f32);
                let ctm = video_timeline.get_screen_ctm().unwrap().inverse().unwrap();
                let k = p.matrix_transform(&ctm);
                let x = k.x();
                model.cursor_right_pos = x as f64;
                log!("Right Pos: ", model.cursor_right_pos);
            }
        }
        Msg::GrabbedRight => {
            log!("Grabbed Right!");
            model.moving_right_cursor = true;
        }
        Msg::SetTimeZoom(event) => {
            let el = model.video.get().expect("No video el");
            let total_duration = el.duration();
            let x = event.client_x();
            let y = event.client_y();
            let video_timeline = model.video_timeline.get().unwrap();
            let p = video_timeline.create_svg_point();
            p.set_x(x as f32);
            p.set_y(y as f32);
            let ctm = video_timeline.get_screen_ctm().unwrap().inverse().unwrap();
            let k = p.matrix_transform(&ctm);
            let x = k.x() as f64;
            // 12.5 up to 87.5
            // 12.5 => left_cursor/100 * total_duration
            // 87.5 => right_cursor/100 * total_duration
            // x => ((x-12.5)/(87.5-12.5)/100)*total_duration
            let window_percentage_watched = (x-12.5)/(87.5-12.5) as f64;
            let window_left = (model.cursor_left_pos- 5.) * 1.111;
            let window_right = (model.cursor_right_pos - 5.) * 1.111;
            let w = window_left + (window_right-window_left)*window_percentage_watched;
            let w = w/100.;
            model.percentage_watched = w;
            el.set_current_time(model.percentage_watched * total_duration);
            // model.cursor_left_pos = x as f64;
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let mut lines = vec![];
    for i in 0..=300 {
        let stroke_width = 0.125;
        let x_pos = (i as f32 / 3.0) * 0.9 + stroke_width;
        let perc_bar = i as f64 / 300.;
        let color = if model.percentage_watched > perc_bar {
            "stroke:red; stroke-width:0.125"
        } else {
            "stroke:gray; stroke-width:0.125"
        };
        lines.push(line_![attrs! {
            At::X1 => (x_pos+5.).to_string(),
            At::Y1 => "0",
            At::X2 => (x_pos+5.).to_string(),
            At::Y2 => "100",
            At::Style => color,
        },]);
    }
    let need_move = (model.moving_left_cursor || model.moving_right_cursor);
    // need these "top" event listeners because Safari does not support pointerCapture API :(
    let mut event_listeners = vec![
        pointer_ev("pointerup", Msg::PointerUp),
        pointer_ev("pointerleave", Msg::PointerUp),
    ];
    if need_move {
        event_listeners.push(pointer_ev("pointermove", Msg::PointerMoved));
    }
    let video_timeline = svg![
        el_ref(&model.video_timeline),
        event_listeners,
        attrs! {
            At::Width => "100%",
            At::Height => "60px",
            At::ViewBox => "0 0 100 600",
            At::PreserveAspectRatio => "none",
        },
        style! {St::TouchAction => "none"},
        g![
            // main timeline
            pointer_ev("pointermove", Msg::SetTime),
            rect![attrs! {
                At::X => "5",
                At::Y => "0",
                At::Width => "90",
                At::Height => "100",
                At::FillOpacity => "0.2",
            },],
            lines
        ],
        g![
            // zoom timeline
            rect![
                pointer_ev("pointermove", Msg::SetTimeZoom),
                attrs! {
                    At::X => "12.5",
                    At::Y => "400",
                    At::Width => "75",
                    At::Height => "200",
                    At::FillOpacity => "0.2",
                },
            ],
            // lines
        ],
        g![
            // left zoom lines
            line_![attrs! { // bridge line
                At::X1 => model.cursor_left_pos,
                At::Y1 => "100",
                At::X2 => "13",// pos +  stroke-width/2
                At::Y2 => "400",
                At::Style => "stroke:red; stroke-width:0.15",
            },],
            polygon![
                simple_ev("pointerdown", Msg::GrabbedLeft),
                attrs! {
                    At::Points => format!("{},{} {},{} {},{}", model.cursor_left_pos, 100
                    , model.cursor_left_pos-5., 400, model.cursor_left_pos+5., 400),
                    At::FillOpacity => "0.5",
                },
            ],
        ],
        g![
            // right zoom lines
            line_![attrs! {
                At::X1 => model.cursor_right_pos,
                At::Y1 => "100",
                At::X2 => "87",
                At::Y2 => "400",
                At::Style => "stroke:red; stroke-width:0.15",
            },],
            polygon![
                simple_ev("pointerdown", Msg::GrabbedRight),
                attrs! {
                    At::Points => format!("{},{} {},{} {},{}", model.cursor_right_pos, 100
                    , model.cursor_right_pos-5., 400, model.cursor_right_pos+5., 400),
                    At::FillOpacity => "0.5",
                },
            ],
        ]
    ];

    div![
        style! {St::Width => "80%", St::Overflow => "hidden", St::Margin => "auto"},
        video![
            el_ref(&model.video),
            attrs! {At::Width => "100%", At::Controls => ""},
            source![
                attrs! {At::Src => "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/Sintel.mp4", At::Type => "video/mp4"}
            ]
        ],
        div![attrs! {At::Id => "video_timeline"}, video_timeline],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
