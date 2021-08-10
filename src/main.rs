#![windows_subsystem = "windows"]

mod interop;
mod window_target;

use interop::create_dispatcher_queue_controller_for_current_thread;
use window_target::CompositionDesktopWindowTargetSource;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use bindings::Windows::{
    Foundation::Numerics::{Vector2, Vector3},
    Win32::System::WinRT::{RoInitialize, RO_INIT_SINGLETHREADED},
    UI::{
        Colors,
        Composition::{CompositionGeometry, CompositionShape, Compositor, ContainerVisual, SpriteVisual},
    },
};

use windows::Interface;

fn run() -> windows::Result<()> {
   unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
   let _controller = create_dispatcher_queue_controller_for_current_thread()?;
   
   let event_loop = EventLoop::new();
   let window = WindowBuilder::new().build(&event_loop).unwrap();
   window.set_title("Zero_Game");

   let compositor = Compositor::new()?;
   let target = window.create_window_target(&compositor, false)?;

   let root = compositor.CreateContainerVisual()?;
   root.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
   target.SetRoot(&root)?;

   let window_size = window.inner_size();
   let window_size = Vector2::new(window_size.width as f32, window_size.height as f32);

   //
   let tile_size = Vector2::new(10.0, 10.0);
   let circle_geometry = compositor.CreateEllipseGeometry()?;
   circle_geometry.SetRadius(tile_size)?;
   let circle_geometry: CompositionGeometry = circle_geometry.cast()?;

   let container_shape = compositor.CreateContainerShape()?;
   let shapes = container_shape.Shapes()?;
   let color_brush = compositor.CreateColorBrushWithColor(Colors::Red()?)?;

   let shape = compositor.CreateSpriteShapeWithGeometry(&circle_geometry)?;
   shape.SetFillBrush(&color_brush)?;
   shape.SetOffset(tile_size / 2.0)?;
   shapes.Append(shape)?;
    //

   event_loop.run(move |event, _, control_flow| {
       *control_flow = ControlFlow::Wait;
       match event {
           Event::WindowEvent {
               event: WindowEvent::CloseRequested,
               window_id,
           } if window_id == window.id() => *control_flow = ControlFlow::Exit,
           Event::WindowEvent {
               event: WindowEvent::Resized(size),
               ..
           } => {
               let size = Vector2::new(size.width as f32, size.height as f32);
           }
           Event::WindowEvent {
               event: WindowEvent::CursorMoved { position, .. },
               ..
           } => {
               let point = Vector2::new(position.x as f32, position.y as f32);
           }
           Event::WindowEvent {
               event: WindowEvent::MouseInput { state, button, ..},
               ..
           } => {
               if state == ElementState::Pressed {

               }
           }
           _ => (),
       }
   });
}

fn main() {
    let result = run();

    if let Err(error) = result {
        error.code().unwrap();
    }
}