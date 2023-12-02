use yew::prelude::*;
use yew_router::components::Link;

use crate::content::PostMeta;
use crate::generator::Generated;
use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub post_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)] 
pub enum Posts_List {

    UsingLibSouthProxy, 

}

impl Posts_List {

    pub fn get_title(&self) -> &'static str {
        match self {
            Self::UsingLibSouthProxy => {
                "How to use the USA Library Proxy!"
            },
            _ => { "Unknown" }
        }
    }

    pub fn from_id(id: u64) -> Option<Self> {
        match id {
            0 => Some(Self::UsingLibSouthProxy),
            _ => None 
        }
    }

    pub fn get_summary(&self) -> &'static str {
        match self {
            Self::UsingLibSouthProxy => {
                "Here's how you can use the library's proxy to access the resources available to the university's interent!"
            }
            _ => { "Unknown" }
        }
    }

    pub fn get_content(&self) -> Html {
        match self {
            Self::UsingLibSouthProxy => {
                html! {
                    <div class="section container">
                        <div class="title">
                            <h1 class="title is-1">{ self.get_title() }</h1>
                        </div>
                        <p class="content">
                            {r#"
                            Have you ever noticed that, whenever you are on campus, 
                            you have access to scientific journals and resources way beyond 
                            what you have access to off campus? This is intentional; your tuition dollars go towards the univeristy 
                            paying these companies and journals to grant students access to a greater breadth of resources beyond just
                            google or wikipedia. This is extremely useful, but what if you need access to these resources but happen to be 
                            off campus? Are you just out of luck? Thankfully no. The university's library thankfully hosts a proxy site, 
                            allowing anyone anywhere, with a valid student ID, to use this tool! 
                            "#}       
                        </p> 
                        <h2 class="title is-4">
                            {"How do you use it?"}
                        </h2>
                        <p class="content">
                            {r#"
                            It's very easy to use the university's proxy server. Pick a website, any website! I will pick
                            "#}
                            <a href="https://scholar.google.com">{"scholar.google.com "}</a>
                            {r#"This website is google for the scholarly world! On your regular home internet, you can search through it all you may like, but you won't normally have access to the articles themselves. On the university's interent, however, you have access to a large part of the content listed! To access this from home, you can instead add "#}
                            <b>{".libproxy.usouthal.edu "}</b> 
                            {r#"to the end of whatever website you are trying to access. In this case "#}
                            <a href="https://scholar.google.com.libproxy.usouthal.edu">{"scholar.google.com.libproxy.usouthal.edu "}</a>
                            {r#"this will redirect you to the library's website where it will ask for your student ID and password. Once this is input you now have free access to even more of humanities knowledge!"#}
                          
                        </p>
                    </div>
                }
            }
            _ => { 
                html! {
                    <div>
                    {"nothing to see here!"}
                    </div>
                }
             }
        }
    }

}



pub struct PostCard {
    post: Option<Posts_List>
}
impl Component for PostCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            post: Posts_List::from_id(ctx.props().post_id),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.post = Posts_List::from_id(ctx.props().post_id);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self { post } = self;
        if let Some(p) = post.as_ref() {
            html! {
                <div class="card">
                    <div class="card-content">
                        <p class = "title is-5">
                            <Link<Route> to={Route::Post{ id: ctx.props().post_id} }>
                                { p.get_title() }
                            </Link<Route>>
                        </p> 
                        { p.get_summary() }

                    </div>
                </div>
            }
        } else {
            html! {
                <div class="card">
                    { "Doesn't Exist. This is a bug." }
                </div>
            }
        }
    }
}
