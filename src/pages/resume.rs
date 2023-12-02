use yew::prelude::*;

pub struct Resume;
impl Component for Resume {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let parsed = Html::from_html_unchecked(AttrValue::from(include_str!("./resume_html.html")));
        html! {
            <div>

                // <iframe src="https://raw.githubusercontent.com/lukecashwell-alt/eportfolio/master/resume.pdf">
                // </iframe>
                {parsed}
            </div>
        }
        // html! {
            // <div class="tile is-ancestor is-vertical">
            //     <div class="tile is-child hero">
            //         <div class="hero-body container pb-0">
            //             <h1 class="title is-1">{ "Welcome . . ." }</h1>
            //             <h2 class="subtitle has-text-centered">{ "my name is Luke" }</h2>
            //         </div>
            //     </div>

            //     // <div class="tile is-child">
            //     //     <figure class="image is-3by1">
            //     //         <img alt="A random image for the input term 'yew'." src="https://source.unsplash.com/random/1200x400/?yew" />
            //     //     </figure>
            //     // </div>

            //     <div class="tile is-parent container">
            //         { self.view_info_tiles() }
            //     </div>
            // </div>

            
        // }
    }
}