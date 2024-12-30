use maud::{html, Markup, Render};

pub struct Head;

impl Render for Head {
    fn render(&self) -> Markup {
        html! {
          head {
            meta charset="utf-8";
            meta http-equiv="X-UA-Compatible" content="IE=edge";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { "Remote" }
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.1/css/all.min.css";
            link rel="stylesheet" href="/main.css";
            script src="https://unpkg.com/htmx.org@2.0.3" {};
            script src="https://unpkg.com/htmx-ext-ws@2.0.1/ws.js" {};
            script src="/main.js" {};
          };
        }
    }
}
