// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.

use crate::i18n::Translations;

pub fn translations() -> Translations {
    Translations {
        game_title: "Rustle",
        win_messages: &["¡Buen trabajo!", "Increíble", "¡Bien hecho!"],
        game_copied: "Juego copiado al portapapeles",
        not_enough_letters: "No hay suficientes letras",
        word_not_found: "Palabra no encontrada",
        hard_mode_alert: "¡El modo difícil solo se puede activar al principio!",
        enter: "Intro",
        delete: "Borrar",
        statistics: "Estadísticas",
        guess_distribution: "Distribución de intentos",
        new_word: "Nueva palabra en",
        share: "Compartir",
        share_failure: "No se pueden compartir los resultados. Esta función solo está disponible en contextos seguros (HTTPS) en algunos o todos los navegadores compatibles.",
        transfer: "Transferir",
        transfer_desc: "Haz clic aquí para transferir tus estadísticas a un nuevo dispositivo.",
        total_tries: "Intentos totales",
        success_rate: "Porcentaje de éxito",
        current_streak: "Racha actual",
        best_streak: "Mejor racha",
        discourage_browser: "Estás usando un navegador integrado y podrías experimentar problemas al compartir o guardar tus resultados. Te recomendamos usar el navegador predeterminado de tu dispositivo.",
        datepicker_title: "Elige una fecha pasada",
        datepicker_choose: "Elegir",
        logout: "Cerrar sesión",
    }
}
