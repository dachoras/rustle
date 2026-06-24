// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.

use crate::i18n::Translations;

pub fn translations() -> Translations {
    Translations {
        game_title: "Rustle",
        win_messages: &["Großartig!", "Genial", "Gut gemacht!"],
        game_copied: "Spiel in die Zwischenablage kopiert",
        not_enough_letters: "Nicht genug Buchstaben",
        word_not_found: "Wort nicht gefunden",
        hard_mode_alert: "Der schwere Modus kann nur zu Beginn aktiviert werden!",
        enter: "Eingabe",
        delete: "Löschen",
        statistics: "Statistik",
        guess_distribution: "Verteilung der Versuche",
        new_word: "Neues Wort in",
        share: "Teilen",
        share_failure: "Ergebnisse konnten nicht geteilt werden. Diese Funktion ist nur in sicheren Kontexten (HTTPS) verfügbar.",
        transfer: "Übertragen",
        transfer_desc: "Klicken Sie hier, um Ihre Statistiken auf ein neues Gerät zu übertragen.",
        total_tries: "Gesamtversuche",
        success_rate: "Erfolgsquote",
        current_streak: "Aktuelle Serie",
        best_streak: "Beste Serie",
        discourage_browser: "Sie verwenden einen In-App-Browser und es kann zu Problemen beim Teilen kommen. Wir empfehlen die Verwendung Ihres Standard-Browsers.",
        datepicker_title: "Wählen Sie ein vergangenes Datum",
        datepicker_choose: "Wählen",
        logout: "Abmelden",
    }
}
