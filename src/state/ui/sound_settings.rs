use macroquad::prelude::*;
use crate::state::ui::components::draw_text_with_emoji;

pub fn draw_sound_settings(
    music_volume: f32,
    sfx_volume: f32,
    music_enabled: bool,
    sfx_enabled: bool,
    emoji_font: Option<&Font>,
) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Título
    let title = "🔊 Configurações de Som 🔊";
    let title_size = 36.0;
    let title_dims = measure_text(title, None, title_size as u16, 1.0);
    draw_text_with_emoji(
        title,
        (screen_width - title_dims.width) / 2.0,
        screen_height * 0.1,
        title_size,
        GOLD,
        emoji_font,
    );

    // Instruções
    let instructions = "Clique nas barras para ajustar volume. Espaço/S para ativar/desativar. ESC para voltar.";
    let inst_size = 18.0;
    let inst_dims = measure_text(instructions, None, inst_size as u16, 1.0);
    draw_text(
        instructions,
        (screen_width - inst_dims.width) / 2.0,
        screen_height * 0.2,
        inst_size,
        LIGHTGRAY,
    );

    // Configuração de Música
    let music_label = format!("🎵 Música: {}", if music_enabled { "ON" } else { "OFF" });
    let music_label_size = 24.0;
    let music_label_dims = measure_text(&music_label, None, music_label_size as u16, 1.0);
    draw_text_with_emoji(
        &music_label,
        (screen_width - music_label_dims.width) / 2.0,
        screen_height * 0.35,
        music_label_size,
        if music_enabled { WHITE } else { GRAY },
        emoji_font,
    );

    // Barra de volume da música
    let bar_width = 300.0;
    let bar_height = 20.0;
    let bar_x = (screen_width - bar_width) / 2.0;
    let bar_y = screen_height * 0.4;
    
    // Fundo da barra
    draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);
    // Progresso do volume
    let volume_width = bar_width * music_volume;
    draw_rectangle(bar_x, bar_y, volume_width, bar_height, if music_enabled { BLUE } else { GRAY });
    // Borda da barra
    draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, WHITE);
    
    // Texto do volume
    let volume_text = format!("{:.0}%", music_volume * 100.0);
    let volume_size = 16.0;
    let volume_dims = measure_text(&volume_text, None, volume_size as u16, 1.0);
    draw_text(
        &volume_text,
        bar_x + (bar_width - volume_dims.width) / 2.0,
        bar_y + (bar_height - volume_size) / 2.0,
        volume_size,
        WHITE,
    );

    // Configuração de Efeitos Sonoros
    let sfx_label = format!("🔊 Efeitos: {}", if sfx_enabled { "ON" } else { "OFF" });
    let sfx_label_size = 24.0;
    let sfx_label_dims = measure_text(&sfx_label, None, sfx_label_size as u16, 1.0);
    draw_text_with_emoji(
        &sfx_label,
        (screen_width - sfx_label_dims.width) / 2.0,
        screen_height * 0.55,
        sfx_label_size,
        if sfx_enabled { WHITE } else { GRAY },
        emoji_font,
    );

    // Barra de volume dos efeitos
    let sfx_bar_y = screen_height * 0.6;
    
    // Fundo da barra
    draw_rectangle(bar_x, sfx_bar_y, bar_width, bar_height, DARKGRAY);
    // Progresso do volume
    let sfx_volume_width = bar_width * sfx_volume;
    draw_rectangle(bar_x, sfx_bar_y, sfx_volume_width, bar_height, if sfx_enabled { GREEN } else { GRAY });
    // Borda da barra
    draw_rectangle_lines(bar_x, sfx_bar_y, bar_width, bar_height, 2.0, WHITE);
    
    // Texto do volume
    let sfx_volume_text = format!("{:.0}%", sfx_volume * 100.0);
    let sfx_volume_dims = measure_text(&sfx_volume_text, None, volume_size as u16, 1.0);
    draw_text(
        &sfx_volume_text,
        bar_x + (bar_width - sfx_volume_dims.width) / 2.0,
        sfx_bar_y + (bar_height - volume_size) / 2.0,
        volume_size,
        WHITE,
    );

    // Botão voltar
    let back_text = "ESC: Voltar ao Menu";
    let back_size = 20.0;
    let back_dims = measure_text(back_text, None, back_size as u16, 1.0);
    draw_text(
        back_text,
        (screen_width - back_dims.width) / 2.0,
        screen_height * 0.85,
        back_size,
        ORANGE,
    );
    
} 