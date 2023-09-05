mod board;
mod pathfinder;
mod cell;
mod test;
mod pathfinding_visualizer;
mod pathfinding_nav;


pub fn main() {
    //yew::Renderer::<test::TestApp>::new().render();
    yew::Renderer::<pathfinding_visualizer::PathfindingVisualizer>::new().render();
}