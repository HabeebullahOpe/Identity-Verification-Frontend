use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
}

enum Msg {
    GenerateDID,
    IssueCredential,
    VerifyCredential,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GenerateDID => {
                // Call API to generate DID
            }
            Msg::IssueCredential => {
                // Call API to issue credential
            }
            Msg::VerifyCredential => {
                // Call API to verify credential
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::GenerateDID)>{ "Generate DID" }</button>
                <button onclick=self.link.callback(|_| Msg::IssueCredential)>{ "Issue Credential" }</button>
                <button onclick=self.link.callback(|_| Msg::VerifyCredential)>{ "Verify Credential" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
