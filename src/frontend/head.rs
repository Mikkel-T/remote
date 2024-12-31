use maud::{html, Markup, Render};

struct Css(&'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}

struct Script(&'static str);

impl Render for Script {
    fn render(&self) -> Markup {
        html! {
          script src=(self.0) {};
        }
    }
}

pub struct Head;

impl Render for Head {
    fn render(&self) -> Markup {
        html! {
          head {
            meta charset="utf-8";
            meta http-equiv="X-UA-Compatible" content="IE=edge";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { "Remote" }
            (Css("https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css"))
            (Css("/main.css"))
            (Script("https://cdnjs.cloudflare.com/ajax/libs/htmx/2.0.4/htmx.min.js"))
            (Script("https://unpkg.com/htmx-ext-ws@2.0.2/ws.js"))
            (Script("/main.js"))
          };
        }
    }
}
