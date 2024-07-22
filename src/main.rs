use rumqttc::v5::mqttbytes::QoS;
use tokio::{task, time};

use eframe::egui;
use rumqttc::v5::{AsyncClient, MqttOptions};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use tokio::runtime::Runtime;

#[tokio::main(flavor = "current_thread")]
async fn main() -> eframe::Result<()> {
    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // set options for egui
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((200.0, 200.0)),
        ..eframe::NativeOptions::default()
    };

    // run eframe for egui
    return eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Box::<ExampleApp>::default()),
    );
}

pub struct ExampleApp {
    weight: f64,
    former_weight: f64,

    tx: Sender<f64>,

    flag_rx: Receiver<bool>,
    flag_tx: Sender<bool>,
    one_second_passed: bool,
}

impl ExampleApp {
    fn name() -> &'static str {
        "MQTT-Scale-Mock"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        let url = "localhost";
        let topic = "topic";

        let (tx, rx) = std::sync::mpsc::channel::<f64>();
        task::spawn(async move {
            let mut mqttoptions = MqttOptions::new("", url, 1883);
            mqttoptions.set_keep_alive(Duration::from_secs(5));
            let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
            loop {
                if let Ok(received) = rx.try_recv() {
                    let payload = received.to_string();
                    match client
                        .publish(topic, QoS::AtLeastOnce, false, payload)
                        .await
                    {
                        Ok(_) => (),
                        Err(err) => println!("{}", err.to_string()),
                    };
                }
                let event = eventloop.poll().await;
                match &event {
                    Ok(v) => {
                        //    println!("Event = {v:?}");
                    }
                    Err(e) => {
                        //   println!("Error = {e:?}");
                    }
                };

                time::sleep(Duration::from_secs(1)).await;
            }
        });
        let (flag_tx, flag_rx) = std::sync::mpsc::channel::<bool>();

        Self {
            weight: 0.0,
            former_weight: 0.0,
            tx,
            flag_rx,
            flag_tx,
            one_second_passed: true,
        }
    }
}

fn send_requests(rx: Receiver<f64>) {}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Gewicht:");

            //ui.add(egui::Slider::new(&mut self.weight, 0.0..=200.0).text("Gewicht"));
            ui.add(egui::DragValue::new(&mut self.weight).update_while_editing(false));

            if (self.former_weight != self.weight) && (self.one_second_passed == true) {
                self.one_second_passed = false;
                self.former_weight = self.weight;

                // put the new weight in tx
                self.tx
                    .send(self.weight.clone())
                    .expect("Could not send the weight. Did you check the right Adress and topic?");

                // This is necessary to not overload the test.moquitto server. Might as well use
                // your own to not need this.
                // send flag to false
                let sender = self.flag_tx.clone();
                tokio::spawn(async move {
                    time::sleep(Duration::from_secs(1)).await;
                    //sender.send(true);
                    match sender.send(true) {
                        Ok(_) => (),
                        Err(_) => println!("could net send true"),
                    };
                });
            };

            // if something was send, then this is true again
            match self.flag_rx.try_recv() {
                Ok(_) => self.one_second_passed = true,
                Err(_) => (),
            };

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
        });
    }
}
