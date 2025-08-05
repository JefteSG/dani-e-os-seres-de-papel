use macroquad::prelude::*;
use crate::state::menu::MenuSelection;
use crate::state::ui::components::draw_text_with_emoji;

pub fn draw_menu(selection: &MenuSelection, player_name: &str, is_editing_name: bool, emoji_font: Option<&Font>) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let title = "🐉 Dani e os Seres de Papel 🐉";
    let title_size = 40.0;
    let title_dims = measure_text(title, None, title_size as u16, 1.0);
    draw_text_with_emoji(
        title,
        (screen_width - title_dims.width) / 2.0,
        screen_height * 0.2,
        title_size,
        YELLOW,
        emoji_font,
    );

    // let instructions = "Use ↑ e ↓ para navegar ou clique. Pressione Enter para selecionar.";
    // let inst_size = 20.0;
    // let inst_dims = measure_text(instructions, None, inst_size as u16, 1.0);
    // draw_text(
    //     instructions,
    //     (screen_width - inst_dims.width) / 2.0,
    //     screen_height * 0.3,
    //     inst_size,
    //     WHITE,
    // );

    let name_label = "Nome do Jogador:";
    let name_label_size = 22.0;
    let name_label_dims = measure_text(name_label, None, name_label_size as u16, 1.0);
    draw_text(
        name_label,
        (screen_width - name_label_dims.width) / 2.0,
        screen_height * 0.38,
        name_label_size,
        ORANGE,
    );

    let input_width = 300.0;
    let input_height = 40.0;
    let input_x = (screen_width - input_width) / 2.0;
    let input_y = screen_height * 0.42;
    
    let input_color = if is_editing_name {
        Color::new(0.2, 0.3, 0.8, 0.3)
    } else {
        Color::new(0.3, 0.3, 0.3, 0.5)
    };
    draw_rectangle(input_x, input_y, input_width, input_height, input_color);
    draw_rectangle_lines(input_x, input_y, input_width, input_height, 2.0, WHITE);
    
    let name_text_size = 20.0;
    let display_name = if is_editing_name {
        format!("{}|", player_name)
    } else {
        player_name.to_string()
    };
    let _name_text_dims = measure_text(&display_name, None, name_text_size as u16, 1.0);
    draw_text(
        &display_name,
        input_x + 10.0,
        input_y + input_height / 2.0 + name_text_size / 2.0,
        name_text_size,
        WHITE,
    );
    
    let edit_instruction = if is_editing_name {
        "Digite o nome e pressione Enter para confirmar"
    } else {
        "Clique no campo ou pressione Tab para editar o nome"
    };
    let edit_inst_size = 16.0;
    let edit_inst_dims = measure_text(edit_instruction, None, edit_inst_size as u16, 1.0);
    draw_text(
        edit_instruction,
        (screen_width - edit_inst_dims.width) / 2.0,
        input_y + input_height + 20.0,
        edit_inst_size,
        LIGHTGRAY,
    );

    let start_text = match selection {
        MenuSelection::Start => ">> Iniciar <<",
        MenuSelection::SoundSettings => "Iniciar",
        MenuSelection::Quit => "Iniciar",
    };
    let start_color = match selection {
        MenuSelection::Start => LIME,
        MenuSelection::SoundSettings => GRAY,
        MenuSelection::Quit => GRAY,
    };

    let sound_text = match selection {
        MenuSelection::Start => "Configurações de Som",
        MenuSelection::SoundSettings => ">> Configurações de Som <<",
        MenuSelection::Quit => "Configurações de Som",
    };
    let sound_color = match selection {
        MenuSelection::Start => GRAY,
        MenuSelection::SoundSettings => BLUE,
        MenuSelection::Quit => GRAY,
    };

    let quit_text = match selection {
        MenuSelection::Start => "Sair",
        MenuSelection::SoundSettings => "Sair",
        MenuSelection::Quit => ">> Sair <<",
    };
    let quit_color = match selection {
        MenuSelection::Start => GRAY,
        MenuSelection::SoundSettings => GRAY,
        MenuSelection::Quit => RED,
    };

    let option_size = 30.0;
    let start_dims = measure_text(start_text, None, option_size as u16, 1.0);
    let sound_dims = measure_text(sound_text, None, option_size as u16, 1.0);
    let quit_dims = measure_text(quit_text, None, option_size as u16, 1.0);

    let start_x = (screen_width - start_dims.width) / 2.0;
    let start_y = screen_height * 0.6;
    if matches!(selection, MenuSelection::Start) {
        draw_rectangle(
            start_x - 10.0,
            start_y - 25.0,
            start_dims.width + 20.0,
            option_size + 10.0,
            Color::new(0.0, 1.0, 0.0, 0.1),
        );
    }

    draw_text(start_text, start_x, start_y, option_size, start_color);

    let sound_x = (screen_width - sound_dims.width) / 2.0;
    let sound_y = screen_height * 0.7;
    if matches!(selection, MenuSelection::SoundSettings) {
        draw_rectangle(
            sound_x - 10.0,
            sound_y - 25.0,
            sound_dims.width + 20.0,
            option_size + 10.0,
            Color::new(0.0, 0.0, 1.0, 0.1),
        );
    }

    draw_text(sound_text, sound_x, sound_y, option_size, sound_color);

    let quit_x = (screen_width - quit_dims.width) / 2.0;
    let quit_y = screen_height * 0.8;
    if matches!(selection, MenuSelection::Quit) {
        draw_rectangle(
            quit_x - 10.0,
            quit_y - 25.0,
            quit_dims.width + 20.0,
            option_size + 10.0,
            Color::new(1.0, 0.0, 0.0, 0.1),
        );
    }

    draw_text(quit_text, quit_x, quit_y, option_size, quit_color);
}

/// Detecta qual opção do menu foi clicada
pub fn get_clicked_menu_option(mouse_x: f32, mouse_y: f32) -> Option<MenuSelection> {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let start_y = screen_height * 0.6;
    let start_text = ">> Iniciar <<";
    let option_size = 30.0;
    let start_dims = measure_text(start_text, None, option_size as u16, 1.0);
    let start_x = (screen_width - start_dims.width) / 2.0;

    if mouse_x >= start_x - 20.0
        && mouse_x <= start_x + start_dims.width + 20.0
        && mouse_y >= start_y - 20.0
        && mouse_y <= start_y + option_size + 10.0
    {
        return Some(MenuSelection::Start);
    }

    let sound_y = screen_height * 0.7;
    let sound_text = ">> Configurações de Som <<";
    let sound_dims = measure_text(sound_text, None, option_size as u16, 1.0);
    let sound_x = (screen_width - sound_dims.width) / 2.0;

    if mouse_x >= sound_x - 20.0
        && mouse_x <= sound_x + sound_dims.width + 20.0
        && mouse_y >= sound_y - 20.0
        && mouse_y <= sound_y + option_size + 10.0
    {
        return Some(MenuSelection::SoundSettings);
    }

    let quit_y = screen_height * 0.8;
    let quit_text = ">> Sair <<";
    let quit_dims = measure_text(quit_text, None, option_size as u16, 1.0);
    let quit_x = (screen_width - quit_dims.width) / 2.0;

    if mouse_x >= quit_x - 20.0
        && mouse_x <= quit_x + quit_dims.width + 20.0
        && mouse_y >= quit_y - 20.0
        && mouse_y <= quit_y + option_size + 10.0
    {
        return Some(MenuSelection::Quit);
    }

    None
}

/// Detecta se o campo de nome foi clicado
pub fn is_name_field_clicked(mouse_x: f32, mouse_y: f32) -> bool {
    let screen_width = screen_width();
    let screen_height = screen_height();
    
    let input_width = 300.0;
    let input_height = 40.0;
    let input_x = (screen_width - input_width) / 2.0;
    let input_y = screen_height * 0.42;
    
    mouse_x >= input_x 
        && mouse_x <= input_x + input_width
        && mouse_y >= input_y
        && mouse_y <= input_y + input_height
}