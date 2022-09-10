
use computer::*;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use pixels::{Pixels, SurfaceTexture};

fn draw(frame: &mut [u8], vram: Vec<u8>)
{
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

        let address = 3 * i;
        let r = vram[address];
        let g = vram[address + 1];
        let b = vram[address + 2];
        let rgba= [r, g, b, 255];

        pixel.copy_from_slice(&rgba);
    }
}


fn main()
{
    let mut computer = Computer::new(Some("bios.bin"),
                                     Some("program.bin"),
                                     4 * 1024 * 1024,
                                     800 * 600 * 3);

    let event_loop = EventLoop::new();

    let size = PhysicalSize
    {
        width: 800,
        height: 600,
    };
    let window = WindowBuilder::new()
        .with_title("Super emulator kurwo")
        .with_inner_size(size)
        .build(&event_loop)
        .unwrap();


    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(800, 600, surface_texture).unwrap();

    event_loop.run(move |event, _, control_flow|
        {
            *control_flow = ControlFlow::Wait;
            match event
            {
                Event::WindowEvent
                {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => ()
            }

            if let Event::RedrawRequested(_) = event
            {
                computer.cycle();
                let vram = computer.get_vram();
                draw(pixels.get_frame(), vram);
                if pixels.render().is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
        });
}
