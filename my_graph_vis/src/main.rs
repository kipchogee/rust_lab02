use eframe::{App, CreationContext, Frame};
use egui::Context;
use egui_graphs::{DefaultGraphView, Graph};
use petgraph::stable_graph::StableGraph;

struct MyGraphApp {
    graph_data: Graph<(), ()>,
}

impl MyGraphApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        let mut pet_graph = StableGraph::new();
        let node_a = pet_graph.add_node(());
        let node_b = pet_graph.add_node(());
        let node_c = pet_graph.add_node(());
        pet_graph.add_edge(node_a, node_b, ());
        pet_graph.add_edge(node_b, node_c, ());
        pet_graph.add_edge(node_c, node_a, ());

        let graph_data = Graph::<(), ()>::from(pet_graph);// 使用引用转换

        Self { graph_data }
    }
}

impl App for MyGraphApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut DefaultGraphView::new(&mut self.graph_data));
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Graph Visualization",
        native_options,
        Box::new(|cc| Box::new(MyGraphApp::new(cc))),
    );
}
