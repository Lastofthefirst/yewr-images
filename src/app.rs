use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Url, DragEvent, HtmlImageElement};
use yew::{html, Component, Context, Html};

pub enum Msg{
    Dropped(DragEvent),
    Dragged(DragEvent),
}

pub struct DropImage{
    images: Vec<String>,
}

impl Component for DropImage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{images:vec!()}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message)->bool{ 
        match msg {
            Msg::Dragged(event)=>{
                event.prevent_default();
                false
            },
            Msg::Dropped(event)=> {
                event.prevent_default();
                let data_transfer = event.data_transfer().expect("Failed to data transfer");
                let item_list = data_transfer.items();
                for i in 0..item_list.length() {
                    let item = item_list.get(i).expect("Failed to find an item at {i}");// why are we unwrapping after expect, i thought expect handled that??
                    if item.kind() == "file" {
                     let file =    item.get_as_file().expect("Failed to find a file.").unwrap();// same why unwrap???

                     // create img element

                     let element = document().create_element("img").expect("failed to create img");
                     let img = element.dyn_ref::<HtmlImageElement>().expect("Failed to create img htmlelement");
                     let url = Url::create_object_url_with_blob(&file).expect("Failed to create url for file");
                     img.set_src(&url);
                     img.set_width(100);
                     img.set_height(100);

                     if let Some(images) = document().get_element_by_id("images"){
                         images.append_child(img).expect("Failed to add image to container in dom.");
                     }
                        self.images.push(file.name());
                    }

                }
                true
            },
        }
    }


    fn view(&self, ctx:&Context<Self>) -> Html {
        let link = ctx.link();
        html!{
            <>
            <div class="drop-zone"
            ondragover={link.callback(|e| Msg::Dragged(e))}
            ondrop={link.callback(|e| Msg::Dropped(e))}>
            <p>{"Drag your images here"}</p>
            </div>
            <div id="images"></div>
            <div>{self.images.iter().collect::<Html>()}</div>
            </>
        }
    }
}