use maud::html;

pub fn base_view(content: maud::Markup) -> maud::Markup {
    html! {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { "Todo M.A.D. Stack" }
            script src="https://unpkg.com/htmx.org@1.9.12" {}
            link href="https://cdnjs.cloudflare.com/ajax/libs/bootstrap/5.3.2/css/bootstrap.min.css" rel="stylesheet";
            link href="https://cdnjs.cloudflare.com/ajax/libs/bootstrap-icons/1.11.1/font/bootstrap-icons.min.css" rel="stylesheet";
        }
        body {
            div id="app" class="container"{
                (content)
            }
        }
    }
}