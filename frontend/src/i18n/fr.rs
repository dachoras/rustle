// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.

use crate::i18n::Translations;

pub fn translations() -> Translations {
    Translations {
        game_title: "Rustle",
        win_messages: &["Excellent travail !", "Superbe", "Bien joué !"],
        game_copied: "Partie copiée dans le presse-papiers",
        not_enough_letters: "Pas assez de lettres",
        word_not_found: "Mot non trouvé",
        hard_mode_alert: "Le mode difficile ne peut être activé qu'au début !",
        enter: "Entrée",
        delete: "Effacer",
        statistics: "Statistiques",
        guess_distribution: "Distribution des essais",
        new_word: "Nouveau mot dans",
        share: "Partager",
        share_failure: "Impossible de partager les résultats. Cette fonctionnalité n'est disponible que dans des contextes sécurisés (HTTPS).",
        transfer: "Transférer",
        transfer_desc: "Cliquez ici pour transférer vos statistiques vers un nouvel appareil.",
        total_tries: "Total des essais",
        success_rate: "Taux de réussite",
        current_streak: "Série actuelle",
        best_streak: "Meilleure série",
        discourage_browser: "Vous utilisez un navigateur intégré et pouvez rencontrer des problèmes de partage. Nous vous encourageons à utiliser le navigateur par défaut.",
        datepicker_title: "Choisissez une date passée",
        datepicker_choose: "Choisir",
        logout: "Se déconnecter",
    }
}
