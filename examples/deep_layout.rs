use std::fs::read_to_string;

use anathema::backend::Backend;
use anathema::backend::tui::TuiBackend;
use anathema::component::Component;
use anathema::runtime::Runtime;
use anathema::templates::{Document, ToSourceKind};
use anathema_state::{State, Value};

fn main() {
    let template = read_to_string("examples/templates/deep_layout/deep_layout.aml").unwrap();

    let doc = Document::new("@index");

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);
    builder.template("index", template.to_template()).unwrap();
    builder
        .prototype(
            "comp1",
            "examples/templates/deep_layout/deep_layout_comp.aml",
            || Comp::default(),
            || CompState::default(),
        )
        .unwrap();
    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();
}

#[derive(Default)]
struct Comp;

#[derive(State, Default)]
struct CompState {
    ticks: Value<u32>,
}

impl Component for Comp {
    type Message = ();
    type State = CompState;

    const TICKS: bool = true;

    fn on_tick(
        &mut self,
        state: &mut Self::State,
        _: anathema::component::Children<'_, '_>,
        _: anathema::component::Context<'_, '_, Self::State>,
        _: std::time::Duration,
    ) {
        state.ticks.set(state.ticks.copy_value() + 1);
    }
}
