extern crate mursten;
extern crate mursten_blocks;
extern crate nalgebra;
extern crate rand;

use mursten::{Application, Data, Backend};
use mursten::dummy::DummyBackend;
use mursten_blocks::cursive_renderer::{CursiveRenderer, CursiveView, CursiveContext};
use mursten_blocks::cursive_renderer::cursive::Cursive;
use mursten_blocks::cursive_renderer::cursive::views::*;
use mursten_blocks::cursive_renderer::cursive::menu::*;
use mursten_blocks::cursive_renderer::cursive::traits::*;
use mursten_blocks::events::simple::EventHandler;
use mursten_blocks::events::{EventReceiver, EventEmitter};
use mursten_blocks::events::transport::Address;


pub fn main() {
    let mut cursive_renderer = CursiveRenderer::new("renderer", View::new());
    let action_reducer = ActionReducer.into_updater("reducer");
    cursive_renderer.connect_to(action_reducer.address());

    Application::new(DummyBackend::new())
        .add_updater(action_reducer)
        .add_renderer(cursive_renderer)
        .run(Model::new());
}

struct Model {
    patients: Vec<MedicalRecord>,
}

struct MedicalRecord {
    name: String,
    surname: String,
    age: usize,
    weight: f32,
}

impl Model {
    pub fn new() -> Self {
        Self { 
            patients: vec![
                MedicalRecord {
                    name: "Pedro".to_owned(),
                    surname: "Rodriguez".to_owned(),
                    age: 25,
                    weight: 93.0,
                },
                MedicalRecord {
                    name: "Homar".to_owned(),
                    surname: "Sanchez".to_owned(),
                    age: 38,
                    weight: 84.2,
                },
                MedicalRecord {
                    name: "Carlos".to_owned(),
                    surname: "Gutierrez".to_owned(),
                    age: 31,
                    weight: 103.1,
                }
            ],
        }
    }
}

impl Data for Model { }

struct View {
}

impl View {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone)]
enum Action {
    ChangeName {
        record_position: usize,
        name: String,
        surname: String,
    },
    CreateNew,
    DeleteRecord(usize),
    Quit,
}

impl CursiveView for View {
    type Model = Model;
    type Event = Action;
    fn configure(&mut self, ctx: &mut CursiveContext<Self>) {
        let addr = ctx.address();
        let addr_2 = ctx.address();
        let mut s = ctx.screen();
        s.add_layer(
            Dialog::new()
                .title("Meical Record System 2000")
                .content(
                    LinearLayout::vertical()
                        .with(|list| {
                        })
                        .with_id("records")
                )
        );
        s.menubar()
            .add_subtree(
                "Records",
                MenuTree::new()
                    .leaf("New", move |_| { addr.send(Action::CreateNew); })
                    .leaf("Quit", move |_| { addr_2.send(Action::Quit); })
            );
        s.set_autohide_menu(false);
    }
    fn update(&mut self, ctx: &mut CursiveContext<Self>, model: &Self::Model) {
        let addr = ctx.address();
        let mut s = ctx.screen();
        s.call_on_id("records", move |ll: &mut LinearLayout| {
            while ll.len() > model.patients.len() { ll.remove_child(0); }
            while ll.len() < model.patients.len() { ll.add_child(PatientView::create()); }
            for (i, record) in model.patients.iter().enumerate() {
                PatientView::bind(record, i, addr.clone(), ll.get_child_mut(i).unwrap().as_any_mut().downcast_mut().unwrap());
            }
        });
    }
}

struct PatientView;

impl PatientView {
    fn create() -> LinearLayout {
        LinearLayout::vertical()
            .child(
                LinearLayout::horizontal()
                    .child(TextView::new("Name: "))
                    .child(TextView::new("_").with_id("name"))
                    .child(TextView::new("  "))
                    .child(TextView::new("Surname: "))
                    .child(TextView::new("_").with_id("surname"))
            )
            .child(
                LinearLayout::horizontal()
                    .child(TextView::new("Age: "))
                    .child(TextView::new("_ years").with_id("age"))
                    .child(TextView::new("  "))
                    .child(TextView::new("Weight: "))
                    .child(TextView::new("_ kg").with_id("weight"))
            )
            .child(
                LinearLayout::horizontal()
                    .child(TextView::new(""))
                    .child(Button::new("Edit", |_| { }).with_id("edit_button"))
                    .child(TextView::new(" "))
                    .child(Button::new("Delete", |_| { }).with_id("delete_button"))
            )
    }
    fn bind(record: &MedicalRecord, record_id: usize, addr: Address<Action>, view: &mut LinearLayout) {
        view.find_id("name",    |tv: &mut TextView| { tv.set_content(record.name.clone()); });
        view.find_id("surname", |tv: &mut TextView| { tv.set_content(record.surname.clone()); });
        view.find_id("age",     |tv: &mut TextView| { tv.set_content(format!("{} years", record.age)); });
        view.find_id("weight",  |tv: &mut TextView| { tv.set_content(format!("{} kg", record.weight)); });

        view.find_id(
            "edit_button",
            |b: &mut Button| {
                b.set_callback(move |_| { });
            }
        );

        view.find_id(
            "delete_button",
            |b: &mut Button| {
                b.set_callback(move |_| { 
                    addr.send(Action::DeleteRecord(record_id));
                });
            }
        );
    }
}

struct ActionReducer;

impl EventHandler for ActionReducer {
    type Model = Model;
    type Backend = DummyBackend<Self::Model>;
    type Event = Action;
    fn handle_event(
        &mut self,
        backend: &mut Self::Backend,
        model: &mut Self::Model,
        event: Self::Event
    ) {
        match event {
            Action::Quit => {
                backend.quit();
            },
            Action::ChangeName { .. } => {
                //use rand::Rng;
                //let names = vec!["Pedro", "Juan", "Jose", "Carlos", "Eucebio", "Ignacio"];
                //model.name = rand::thread_rng().choose(&names).unwrap().to_string();
            },
            Action::CreateNew => {
                model.patients.push(
                    MedicalRecord {
                        name: "Eucebio".to_owned(),
                        surname: "Valente".to_owned(),
                        age: 72,
                        weight: 67.8,
                    }
                )
            },
            Action::DeleteRecord(record_id) => {
                model.patients.remove(record_id);
            },
        }
    }
}

