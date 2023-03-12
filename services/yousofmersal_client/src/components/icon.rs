use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct IconProps<'a> {
    width: u32,
    height: u32,
    class: Option<&'a str>
}

pub fn Logo<'a>(cx: Scope<'a, IconProps>) -> Element<'a> {
    cx.render(rsx!{
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 300 300",
            width: "{cx.props.width}px",
            height: "{cx.props.height}px",
            class: cx.props.class.unwrap_or(""),
            path {
                style: "fill:#303841;fill-opacity:1;stroke:#303841;stroke-width:0.556144;stroke-dasharray:none;stroke-opacity:1;-inkscape-stroke:none;stop-color:#000000;",
                d: "M 2.1694791 -100.49425 C 13.338364 -73.520227 24.522587 -46.520103 35.706398 -19.520464 L 71.335133 -71.845035 L 2.1694791 -100.49425 z M -1.2668234 -100.4939 L -70.440214 -71.841167 L -34.871618 -19.364316 L -1.2668234 -100.4939 z M 0.45291044 -98.369379 L -33.258444 -16.98412 L 0.28304716 32.502011 L 34.091115 -17.148356 C 22.873579 -44.230588 11.656282 -71.312292 0.45291044 -98.369379 z M 72.993674 -70.014521 L 36.805059 -16.867713 C 44.761778 2.3411522 52.712578 21.53983 60.664695 40.740908 L 101.67806 -0.27913943 L 102.02165 0.064807334 L 72.993674 -70.014521 z M -72.094887 -70.002915 L -101.12005 0.070082592 L -100.78666 -0.26296197 L -59.777513 40.763767 L -35.972036 -16.708048 L -72.094887 -70.002915 z M 35.189775 -14.495606 L 1.7323361 34.639897 L 28.006633 73.404245 L 58.827147 42.578456 C 50.94896 23.555481 43.072346 4.5350938 35.189775 -14.495606 z M -34.358511 -14.327852 L -57.939965 42.602019 L -27.39904 73.155956 L -1.170462 34.636732 L -34.358511 -14.327852 z M 100.82839 3.9674425 L 61.659256 43.142557 C 65.482064 52.373185 69.309654 61.612005 73.129775 70.838008 L 100.82839 3.9674425 z M -99.905338 4.0142165 L -72.230285 70.827457 L -60.772075 43.164713 L -99.905338 4.0142165 z M 0.27847527 36.77497 L -25.671218 74.884482 L 0.42407237 100.99103 L 26.277756 75.133122 L 0.27847527 36.77497 z M 59.82206 44.980104 L 29.378552 75.428537 L 37.288624 87.098812 L 71.4118 72.96464 C 67.552021 63.642753 63.684691 54.307085 59.82206 44.980104 z M -58.934527 45.002964 L -70.515475 72.961475 L -36.781261 86.934927 L -28.774475 75.176379 L -58.934527 45.002964 z M -27.047005 76.904905 L -34.515362 87.87322 L -3.0597074 100.90275 L -27.047005 76.904905 z M 27.649674 77.157414 L 3.8691669 100.94179 L 35.02378 88.037104 L 27.649674 77.157414 z ",
                transform: "matrix(1.4694004,0,0,1.4694004,149.33619,149.15406)"
            }
            path {
                d: "M 27.47865,255.49297",
                style: "fill:none;stroke:#000000;stroke-width:3.52778;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:1"
            }
        }
    })
}

pub fn Logo_c<'a>(cx: Scope<'a, IconProps>) -> Element<'a> {
    cx.render(rsx!{
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 300 300",
            width: "{cx.props.width}px",
            height: "{cx.props.height}px",
            class: cx.props.class.unwrap_or(""),
            path {
                class: "dark:fill-slate-100 fill-ico ",
                stroke: "#303841",
                stroke_width: ".56",
                d: "M2.17-100.5 35.7-19.51l35.63-52.33-69.17-28.64zm-3.44 0-69.17 28.66 35.57 52.48 33.6-81.13zm1.72 2.13-33.7 81.39L.27 32.5 34.1-17.15.45-98.37zM73-70 36.81-16.87l23.85 57.61L101.68-.28l.34.34L73-70zm-145.08 0L-101.12.08l.33-.33 41.01 41.02 23.8-57.47-36.1-53.3zM35.19-14.5 1.73 34.64 28.01 73.4l30.82-30.82L35.19-14.5zm-69.55.17L-57.94 42.6l30.54 30.56 26.23-38.52-33.19-48.97zm135.19 18.3L61.66 43.14l11.47 27.7 27.7-66.87zM-99.91 4l27.68 66.82 11.46-27.67L-99.91 4zM.28 36.77l-25.95 38.11L.43 101l25.85-25.86-26-38.36zm59.54 8.21L29.38 75.43l7.9 11.67 34.13-14.14-11.59-27.98zM-58.93 45l-11.59 27.96 33.74 13.97 8-11.75L-58.92 45zm31.88 31.9-7.47 10.97 31.46 13.03-23.99-24zm54.7.26L3.87 100.94l31.15-12.9-7.37-10.88z",
                style: "-inkscape-stroke:none;",
                transform: "matrix(1.5 0 0 1.5 149.34 149.15)"
            }
        }
    })
}
