use bevy::{
    a11y::AccessibilityPlugin,
    prelude::*,
    render::{
        render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext, RenderLabel},
        renderer::RenderContext,
        view::ExtractedWindows,
        RenderApp, RenderPlugin,
    },
    window::PresentMode,
    winit::WinitPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            WindowPlugin {
                primary_window: Some(Window {
                    title: "minimal bevy_render".into(),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            },
            // This plugin is required by the winit plugin
            AccessibilityPlugin,
            AssetPlugin::default(),
            WinitPlugin::default(),
            RenderPlugin::default(),
            // NOTE: Load this after renderer initialization so that it knows about the supported
            // compressed texture formats
            ImagePlugin::default(),
            CustomRendererPlugin,
        ))
        .run();
}

struct CustomRendererPlugin;
impl Plugin for CustomRendererPlugin {
    fn build(&self, _app: &mut App) {}
    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            panic!("render_app not found");
        };

        let mut graph = render_app.world.resource_mut::<RenderGraph>();
        graph.add_node(CustomRenderNodeLabel, CustomRenderNode);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, RenderLabel)]
struct CustomRenderNodeLabel;

struct CustomRenderNode;
impl Node for CustomRenderNode {
    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let Some((_e, window)) = world.resource::<ExtractedWindows>().iter().next() else {
            println!("no window found");
            return Ok(());
        };

        let Some(texture_view) = window.swap_chain_texture_view.as_ref() else {
            println!("no swap chain texture view");
            return Ok(());
        };

        let command_encoder = render_context.command_encoder();

        // Clear the screen
        let _render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Custom Clear Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        println!("draw!");

        Ok(())
    }
}
