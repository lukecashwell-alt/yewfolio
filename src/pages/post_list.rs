use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pagination::{PageQuery, Pagination};
use crate::components::post_card::PostCard;
use crate::Route;

const ITEMS_PER_PAGE: u64 = 10;
const TOTAL_PAGES: u64 = u64::MAX / ITEMS_PER_PAGE;

pub enum Msg {
    PageUpdated,
}

pub struct PostList {
    page: u64,
    _listener: LocationHandle,
}

fn current_page(ctx: &Context<PostList>) -> u64 {
    let location = ctx.link().location().unwrap();

    location.query::<PageQuery>().map(|it| it.page).unwrap_or(1)
}

impl Component for PostList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let listener = ctx
            .link()
            .add_location_listener(link.callback(move |_| Msg::PageUpdated))
            .unwrap();

        Self {
            page: current_page(ctx),
            _listener: listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PageUpdated => self.page = current_page(ctx),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page;

        html! {
            <div class="section container">
                <h1 class="title">{ "Posts" }</h1>
                <h2 class="subtitle">{ "Look at these neat looking posts!" }</h2>
                { self.view_posts(ctx) }
                // <Pagination
                //     {page}
                //     total_pages={TOTAL_PAGES}
                //     route_to_page={Route::Posts}
                // />
            </div>
        }
    }
}
impl PostList {
    fn view_posts(&self, _ctx: &Context<Self>) -> Html {
        // let start_seed = (self.page - 1) * ITEMS_PER_PAGE;
        let mut cards = (0..1).map(|id| {
            html! {
                <li class="list-item mb-5">
                    <PostCard  post_id={id}   />
                </li>
            }
        });
        html! {
            <div class="columns">
                <div class="column">
                    <ul class="list">
                        { for cards }
                    </ul>
                </div> 
            </div>
        }
    }
}
