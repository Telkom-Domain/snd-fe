use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value, from_value, Map};
use crate::types::var;

pub enum Msg {
    ToggleDeleteCard,
    RequestDeleteCard,
    Ignore,
}


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowDeleteCardProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_delete_card: bool,
    #[prop_or_default]
    pub delete_index: String,
    pub on_toggle_deletecard:Callback<Msg>,

    #[prop_or_default]
    pub card_index: String,
    // #[prop_or_default]
    // pub modal_open_record: bool,
}


pub struct DeleteCard {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowDeleteCardProps,
    callback_toggle_deletecard: Callback<Msg>,
    fetch_task: Option<FetchTask>,
}

impl Component for DeleteCard {
    type Message = Msg;
    type Properties = WindowDeleteCardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_deletecard: props.on_toggle_deletecard.clone(),
            props,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDeleteCard => {
                self.callback_toggle_deletecard.emit(Msg::ToggleDeleteCard);
                // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_record));
                // ConsoleService::info(&format!("DEBUG : self.delete_index MODAL COMP:{:?}", self.props.delete_index.clone()));
                // ConsoleService::info(&format!("DEBUG : self.card_index MODAL COMP:{:?}", self.props.card_index.clone()));
                true
            }

            Msg::RequestDeleteCard => {
                let url = format!("https://search-discovery-api.dev-domain.site/api/document/{}/{}", &self.props.card_index, &self.props.delete_index);
                // ConsoleService::info(&format!("RecordID: {:?}", self.props.delete_index));
                let request = Request::delete(url)
                    // .header("Content-Type", "application/json")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        Msg::Ignore
                    });
        
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);
                true
            }

            Msg::Ignore => {
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.delete_index != props.delete_index {
            self.props.delete_index = props.delete_index;
            true 
        // } else if self.props.modal_open_record != props.modal_open_record {

        //     self.props.modal_open_record = props.modal_open_record;
        //     self.link.send_message(Msg::ToggleDeleteCard);
        //     // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_record));
        //     true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"DELETE RECORD "}{self.props.delete_index.clone()}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleDeleteCard)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h6>{"Are you sure?"}
                     </h6>

                    <button disabled=true class="window-delete-warning">
                        {"THIS OPERATION CANNOT BE REVERSED OR UNDONE!"}
                    </button> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::RequestDeleteCard)
                        // onchange= self.link.callback(|_| Msg::ToggleDeleteCard)

                        // onclick=self.link.batch_callback(|_| vec![
                        //     Msg::RequestDeleteCard, 
                        //     Msg::ToggleDeleteCard, 
                        // ])
                    >
                        { "DELETE RECORD" }
                    </button>
                    
                </div>

            </div>
        }
    }
}

impl DeleteCard {
    fn modal_success(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"DELETE RECORD SUCCESSFUL"}</h1>
                    </div> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleDeleteCard)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }
}