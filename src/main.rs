use anyhow::Result;
use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::prelude::*;
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};
use sysinfo::System;
use tokio::time::sleep;
use colorgrad::{self};

/// LiveScope - Real-time System Performance Art Visualizer
#[derive(Parser)]
#[command(name = "livescope")]
#[command(about = "A mesmerizing real-time system performance art visualizer")]
struct Args {
    /// Refresh rate in milliseconds (default: 16ms for ~60fps)
    #[arg(short, long, default_value = "16")]
    refresh: u64,
    
    /// Enable particle effects for network activity
    #[arg(short, long)]
    particles: bool,
    
    /// Color theme (fire, ocean, matrix, rainbow)
    #[arg(short, long, default_value = "fire")]
    theme: String,
}

struct LiveScope {
    system: System,
    width: u16,
    height: u16,
    cpu_history: Vec<Vec<f32>>,
    memory_wave: Vec<f32>,
    particles: Vec<Particle>,
    particles_enabled: bool,
    gradient: colorgrad::Gradient,
    rng: ThreadRng,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    char: char,
}

impl LiveScope {
    fn new(theme: &str, particles_enabled: bool) -> Result<Self> {
        let (width, height) = size()?;
        let mut system = System::new_all();
        system.refresh_all();
        
        let cpu_count = system.cpus().len();
        let cpu_history = vec![vec![0.0; width as usize]; cpu_count];
        let memory_wave = vec![0.0; width as usize];
        
        let gradient = match theme {
            "fire" => colorgrad::turbo(),
            "ocean" => colorgrad::viridis(), 
            "matrix" => colorgrad::CustomGradient::new()
                .colors(&[
                    colorgrad::Color::from_html("#000000").unwrap(),
                    colorgrad::Color::from_html("#00ff00").unwrap(),
                ])
                .build()?,
            "rainbow" => colorgrad::rainbow(),
            _ => colorgrad::turbo(),
        };
        
        Ok(Self {
            system,
            width,
            height,
            cpu_history,
            memory_wave,
            particles: Vec::new(),
            particles_enabled,
            gradient,
            rng: thread_rng(),
        })
    }
    
    fn update(&mut self) {
        self.system.refresh_cpu();
        self.system.refresh_memory();
        
        // Update CPU history (shift left and add new values)
        for (i, cpu) in self.system.cpus().iter().enumerate() {
            if i < self.cpu_history.len() {
                self.cpu_history[i].rotate_left(1);
                if let Some(last) = self.cpu_history[i].last_mut() {
                    *last = cpu.cpu_usage();
                }
            }
        }
        
        // Update memory wave
        let memory_percent = self.system.used_memory() as f64 / self.system.total_memory() as f64;
        self.memory_wave.rotate_left(1);
        if let Some(last) = self.memory_wave.last_mut() {
            *last = memory_percent as f32;
        }
        
        // Update particles
        self.particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 0.02;
            p.vy += 0.1; // gravity
            p.life > 0.0 && p.x >= 0.0 && p.x < self.width as f32 && p.y >= 0.0 && p.y < self.height as f32
        });
        
        // Spawn new particles based on network activity
        if self.particles_enabled && self.rng.gen_bool(0.3) {
            self.particles.push(Particle {
                x: self.rng.gen_range(0.0..self.width as f32),
                y: 0.0,
                vx: self.rng.gen_range(-0.5..0.5),
                vy: self.rng.gen_range(0.1..0.5),
                life: 1.0,
                char: ['●', '○', '◆', '◇', '★', '☆'][self.rng.gen_range(0..6)],
            });
        }
    }
    
    fn render(&mut self) -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        
        // Render CPU patterns
        let cpu_section_height = self.height / 3;
        for row in 0..cpu_section_height {
            for col in 0..self.width {
                let intensity = self.calculate_cpu_intensity(col, row);
                let color = self.gradient.at(intensity as f64).to_rgba8();
                let char = self.get_pattern_char(intensity);
                
                execute!(
                    stdout,
                    MoveTo(col, row),
                    SetForegroundColor(Color::Rgb { r: color[0], g: color[1], b: color[2] }),
                    Print(char)
                )?;
            }
        }
        
        // Render memory waves
        let wave_start = cpu_section_height;
        let wave_height = self.height / 3;
        for row in 0..wave_height {
            for col in 0..self.width {
                let wave_y = self.calculate_memory_wave(col as usize, row, wave_height);
                let intensity = if row == wave_y { 1.0 } else { 0.0 };
                
                if intensity > 0.0 {
                    let color = self.gradient.at(0.7).to_rgba8();
                    execute!(
                        stdout,
                        MoveTo(col, wave_start + row),
                        SetForegroundColor(Color::Rgb { r: color[0], g: color[1], b: color[2] }),
                        Print('▓')
                    )?;
                }
            }
        }
        
        // Render particles (only if enabled)
        if self.particles_enabled {
            for particle in &self.particles {
                if particle.life > 0.0 {
                    let color = self.gradient.at(particle.life as f64).to_rgba8();
                    execute!(
                        stdout,
                        MoveTo(particle.x as u16, particle.y as u16),
                        SetForegroundColor(Color::Rgb { r: color[0], g: color[1], b: color[2] }),
                        Print(particle.char)
                    )?;
                }
            }
        }
        
        // Render info panel
        self.render_info_panel(&mut stdout)?;
        
        execute!(stdout, ResetColor)?;
        stdout.flush()?;
        Ok(())
    }
    
    fn calculate_cpu_intensity(&self, col: u16, row: u16) -> f32 {
        if self.cpu_history.is_empty() || col as usize >= self.cpu_history[0].len() {
            return 0.0;
        }
        
        let cpu_index = (row as usize * self.cpu_history.len()) / (self.height as usize / 3);
        let cpu_index = cpu_index.min(self.cpu_history.len() - 1);
        
        self.cpu_history[cpu_index][col as usize] / 100.0
    }
    
    fn calculate_memory_wave(&self, col: usize, _row: u16, wave_height: u16) -> u16 {
        if col >= self.memory_wave.len() {
            return wave_height / 2;
        }
        
        let base_wave = (self.memory_wave[col] * wave_height as f32) as u16;
        let time_offset = col as f32 * 0.1;
        let wave_offset = (time_offset.sin() * 3.0) as i16;
        
        ((base_wave as i16 + wave_offset).max(0).min(wave_height as i16 - 1)) as u16
    }
    
    fn get_pattern_char(&self, intensity: f32) -> char {
        match (intensity * 8.0) as u8 {
            0 => ' ',
            1 => '░',
            2 => '▒',
            3 => '▓',
            4 => '█',
            5 => '▀',
            6 => '▄',
            7 => '▌',
            _ => '█',
        }
    }
    
    fn toggle_particles(&mut self) {
        self.particles_enabled = !self.particles_enabled;
        if !self.particles_enabled {
            self.particles.clear(); // Clear existing particles when disabled
        }
    }
    
    fn render_info_panel(&mut self, stdout: &mut std::io::Stdout) -> Result<()> {
        let info_y = self.height - 5;
        let cpu_usage: f32 = self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / self.system.cpus().len() as f32;
        let memory_percent = (self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0) as u8;
        
        let particle_status = if self.particles_enabled { "ON" } else { "OFF" };
        execute!(
            stdout,
            MoveTo(2, info_y),
            SetForegroundColor(Color::White),
            Print(format!("LiveScope v0.1.0 | CPU: {:.1}% | RAM: {}% | Particles: {} [{}]", 
                         cpu_usage, memory_percent, self.particles.len(), particle_status))
        )?;
        
        execute!(
            stdout,
            MoveTo(2, info_y + 1),
            Print("Press 'q' to quit, 'p' to toggle particles")
        )?;
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    
    let mut livescope = LiveScope::new(&args.theme, args.particles)?;
    let refresh_duration = Duration::from_millis(args.refresh);
    
    loop {
        let start = Instant::now();
        
        // Handle input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('p') => {
                            livescope.toggle_particles();
                        }
                        _ => {}
                    }
                }
            }
        }
        
        livescope.update();
        livescope.render()?;
        
        let elapsed = start.elapsed();
        if elapsed < refresh_duration {
            sleep(refresh_duration - elapsed).await;
        }
    }
    
    execute!(stdout(), LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    println!("LiveScope terminated. Thanks for watching the show!");
    
    Ok(())
}
