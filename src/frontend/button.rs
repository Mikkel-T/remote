use maud::{html, Markup, Render};

pub struct Button {
    name: &'static str,
    icon: &'static str,
    icon2: Option<&'static str>,
    double: bool,
}

impl Button {
    pub fn new(name: &'static str, icon: &'static str) -> Button {
        Button {
            name,
            icon,
            icon2: None,
            double: false,
        }
    }

    pub fn double(&mut self) -> &mut Self {
        self.double = true;
        self
    }

    pub fn icon2(&mut self, icon2: &'static str) -> &mut Self {
        self.icon2 = Some(icon2);
        self
    }

    fn inner(&self) -> Markup {
        html! {
            @if let Some(icon2) = self.icon2 {
                i.fas.{"fa-" (self.icon)} { "" }
                i.fas.{"fa-" (icon2)} { "" }
            } @else {
                i.fas.{"fa-" (self.icon)} { "" }
            }
        }
    }
}

impl Render for Button {
    fn render(&self) -> Markup {
        html! {
            @if self.double {
                button.double ws-send name=(self.name) {
                    ( self.inner() )
                }
            } @else {
                button ws-send name=(self.name) {
                    ( self.inner() )
                }
            }
        }
    }
}
