use macroquad::prelude::*;

pub fn draw_game_over(winner: &Option<String>) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Fundo escuro semi-transparente
    draw_rectangle(
        0.0,
        0.0,
        screen_width,
        screen_height,
        Color::new(0.0, 0.0, 0.0, 0.8),
    );

    // Painel central
    let panel_width = 400.0;
    let panel_height = 300.0;
    let panel_x = (screen_width - panel_width) / 2.0;
    let panel_y = (screen_height - panel_height) / 2.0;

    // Fundo do painel
    draw_rectangle(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        Color::new(0.1, 0.1, 0.2, 0.95),
    );
    draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 3.0, GOLD);

    // Título principal
    let title_text = "GAME OVER";
    let title_size = 36.0;
    let title_dims = measure_text(title_text, None, title_size as u16, 1.0);
    draw_text(
        title_text,
        panel_x + (panel_width - title_dims.width) / 2.0,
        panel_y + 60.0,
        title_size,
        RED,
    );

    // Resultado da partida
    if let Some(ref winner) = winner {
        let (result_text, result_color) = if winner == "Jogador" {
            ("VITORIA!", GREEN)
        } else {
            ("DERROTA!", RED)
        };

        let result_size = 28.0;
        let result_dims = measure_text(result_text, None, result_size as u16, 1.0);
        draw_text(
            result_text,
            panel_x + (panel_width - result_dims.width) / 2.0,
            panel_y + 110.0,
            result_size,
            result_color,
        );

        // Detalhes do vencedor
        let winner_detail = if winner == "Jogador" {
            "Parabens! Voce derrotou o inimigo!"
        } else {
            "O inimigo foi mais forte desta vez..."
        };

        let detail_size = 18.0;
        let detail_dims = measure_text(winner_detail, None, detail_size as u16, 1.0);
        draw_text(
            winner_detail,
            panel_x + (panel_width - detail_dims.width) / 2.0,
            panel_y + 150.0,
            detail_size,
            WHITE,
        );
    }

    // Botão de voltar
    let button_width = 200.0;
    let button_height = 40.0;
    let button_x = panel_x + (panel_width - button_width) / 2.0;
    let button_y = panel_y + 200.0;

    // Verificar hover no botão
    let (mouse_x, mouse_y) = mouse_position();
    let is_button_hovered = mouse_x >= button_x
        && mouse_x <= button_x + button_width
        && mouse_y >= button_y
        && mouse_y <= button_y + button_height;

    let button_color = if is_button_hovered {
        Color::new(0.3, 0.3, 0.6, 1.0)
    } else {
        Color::new(0.2, 0.2, 0.4, 1.0)
    };

    draw_rectangle(
        button_x,
        button_y,
        button_width,
        button_height,
        button_color,
    );
    draw_rectangle_lines(button_x, button_y, button_width, button_height, 2.0, WHITE);

    let button_text = "ESCOLHER OUTRO INIMIGO";
    let button_text_size = 16.0;
    let button_text_dims = measure_text(button_text, None, button_text_size as u16, 1.0);
    draw_text(
        button_text,
        button_x + (button_width - button_text_dims.width) / 2.0,
        button_y + (button_height + button_text_size) / 2.0,
        button_text_size,
        WHITE,
    );

    // Instruções menores
    let instructions = "ESC ou clique no botao para continuar";
    let inst_size = 14.0;
    let inst_dims = measure_text(instructions, None, inst_size as u16, 1.0);
    draw_text(
        instructions,
        panel_x + (panel_width - inst_dims.width) / 2.0,
        panel_y + 260.0,
        inst_size,
        LIGHTGRAY,
    );
}