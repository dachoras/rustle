// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.

use crate::i18n::Translations;

pub fn translations() -> Translations {
    Translations {
        game_title: "Rustle",
        win_messages: &["Ótimo trabalho!", "Incrível", "Bem feito!"],
        game_copied: "Jogo copiado para a área de transferência",
        not_enough_letters: "Letras insuficientes",
        word_not_found: "Palavra não encontrada",
        hard_mode_alert: "O modo difícil só pode ser ativado no início!",
        enter: "Enviar",
        delete: "Apagar",
        statistics: "Estatísticas",
        guess_distribution: "Distribuição de tentativas",
        new_word: "Nova palavra em",
        share: "Compartilhar",
        share_failure: "Não foi possível compartilhar os resultados. Este recurso está disponível apenas em contextos seguros (HTTPS).",
        transfer: "Transferir",
        transfer_desc: "Clique aqui para transferir suas estatísticas para um novo dispositivo.",
        total_tries: "Tentativas totais",
        success_rate: "Taxa de sucesso",
        current_streak: "Sequência atual",
        best_streak: "Melhor sequência",
        discourage_browser: "Você está usando um navegador integrado e pode ter problemas para compartilhar. Recomendamos usar o navegador padrão do seu dispositivo.",
        datepicker_title: "Escolha uma data anterior",
        datepicker_choose: "Escolher",
        logout: "Sair",
    }
}
