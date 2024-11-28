use gpui::*;
use nostr_sdk::prelude::*;
use state::get_client;

pub mod state;

const TEST_USER: &str = "npub1zfss807aer0j26mwp2la0ume0jqde3823rmu97ra6sgyyg956e0s6xw445";

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

#[tokio::main]
async fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.background_executor()
            .spawn(async move {
                let client = get_client().await;
                let public_key = PublicKey::from_bech32(TEST_USER).unwrap();
                let filter = Filter::new().kind(Kind::GiftWrap).pubkey(public_key);
                let events = client.fetch_events(vec![filter], None).await.unwrap();

                println!("Events: {}", events.len())
            })
            .detach();

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        })
        .unwrap();
    });
}
