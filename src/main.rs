use yew::prelude::*;

enum Msg {
    AddOne,
}

struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div>

            <div class="container mx-auto">

                <div class="flex justify-center py-5">
                <p class="text-7xl">{self.count}</p>
                </div>

                <div class="flex justify-center width-full my-5">
                <div class="flex justify-center items-center bg-black text-white text-5xl h-24 w-96 hover:bg-slate-800" onclick={link.callback(|_| Msg::AddOne)}>
                {"+1"}
                </div>
                </div>
               
              
                <div>
                <p class="hey bg-red-500 text-white px-10 py-14 text-7xl text-center">{"Hi everyone!!!"}</p>
                </div>

            </div>
       
            </div>
        }
    }
}
fn main() {
    println!("Hello, world!");
    yew::Renderer::<CounterComponent>::new().render();
}

