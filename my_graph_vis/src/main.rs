use eframe::{App, CreationContext, NativeOptions};
use egui::{CentralPanel, Context};
use egui_graphs::{DefaultGraphView, Graph}; // 默认的图表视图
use petgraph::stable_graph::StableGraph; // 用于创建图数据结构

// 定义我们的应用程序结构体
struct MyLab2App {
    graph_visualization_data: Graph<(), ()>, // 节点和边不携带额外数据，所以是 ()
}

impl MyLab2App {
    // 应用程序的构造函数
    fn new(_cc: &CreationContext<'_>) -> Self {
        // 1. 使用 petgraph 创建图的逻辑结构
        let mut pet_graph: StableGraph<(), ()> = StableGraph::new();

        let node_a = pet_graph.add_node(()); // 添加节点 A
        let node_b = pet_graph.add_node(()); // 添加节点 B
        let node_c = pet_graph.add_node(()); // 添加节点 C

        pet_graph.add_edge(node_a, node_b, ()); // 添加边 A -> B
        pet_graph.add_edge(node_b, node_c, ()); // 添加边 B -> C
        pet_graph.add_edge(node_c, node_a, ()); // 添加边 C -> A (形成一个环)

        // 2. 将 petgraph 的图转换为 egui_graphs::Graph 以便显示
        let graph_visualization_data = Graph::from(&pet_graph);

        Self {
            graph_visualization_data,
        }
    }
}

// 实现 eframe::App Trait，这是 egui 应用程序的核心
impl App for MyLab2App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // 创建一个中央面板来放置我们的图表
        CentralPanel::default().show(ctx, |ui| {
            // 3. 将 GraphView widget 添加到 UI 中
            // DefaultGraphView 使用默认的节点和边形状以及默认的随机布局
            ui.add(&mut DefaultGraphView::new(
                &mut self.graph_visualization_data,
            ));
        });
    }
}

// 主函数，应用程序的入口
fn main() {
    let native_options = NativeOptions::default(); // 使用默认的 eframe 配置

    // 启动 eframe 应用程序
    eframe::run_native(
        "Lab2 Graph Visualization", // 窗口标题
        native_options,
        Box::new(|cc| Ok(Box::new(MyLab2App::new(cc)))), // 创建我们的 App 实例
    )
    .expect("Failed to run eframe application"); // 如果启动失败则 panic
}
