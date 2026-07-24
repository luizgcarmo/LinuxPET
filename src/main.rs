use image::imageops::FilterType;
use image::GenericImageView;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use tao::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const SCALE_FACTOR: f32 = 0.20;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();

    // 1. Carrega a imagem pet.png da raiz do projeto
    let img_path = "pet.png";
    let original_img = image::open(img_path)
        .expect("Certifique-se de que 'pet.png' está na raiz do projeto (ao lado do Cargo.toml)!");

    // 2. Detecta o tamanho do monitor diretamente pelo event_loop (sem janela temporária)
    let primary_monitor = event_loop
        .primary_monitor()
        .or_else(|| event_loop.available_monitors().next())
        .expect("Não foi possível detectar o monitor");

    let screen_height = primary_monitor.size().height as f32;

    // 3. Redimensiona a imagem mantendo a proporção
    let (orig_width, orig_height) = original_img.dimensions();
    let aspect_ratio = orig_width as f32 / orig_height as f32;

    let target_height = (screen_height * SCALE_FACTOR) as u32;
    let target_width = (target_height as f32 * aspect_ratio) as u32;

    let resized_img = original_img.resize(target_width, target_height, FilterType::Lanczos3);
    let (width, height) = resized_img.dimensions();

    // 4. Cria a janela flutuante transparente
    let raw_window = WindowBuilder::new()
        .with_title("Rust Desktop Pet")
        .with_decorations(false)
        .with_transparent(true)
        .with_always_on_top(true)
        .with_inner_size(tao::dpi::LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)
        .expect("Falha ao criar janela!");

    let window: &'static _ = Box::leak(Box::new(raw_window));
    

    // 5. Configura o contexto de pixels
    let surface_texture = SurfaceTexture::new(width, height, window);
    let mut pixels = Pixels::new(width, height, surface_texture)?;

    pixels.clear_color(pixels::wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
    });

    // 6. Copia os bytes RGBA da imagem redimensionada direto para o frame do pixels
    let frame = pixels.frame_mut();
    let rgba_bytes = resized_img.to_rgba8();
    frame.copy_from_slice(&rgba_bytes);

    // 7. Loop de Eventos
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            // Re-renderiza a janela
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                if let Err(err) = pixels.render() {
                    eprintln!("Erro ao renderizar pixels: {:?}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                // Permite arrastar a janela clicando em qualquer pixel do pet
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left && state == ElementState::Pressed {
                        // USA O CLONE AQUI PARA NÃO PRENDER O EMBRÉSTIMO DA JANELA ORIGINAL
                        let _ = window.drag_window();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    });
}