// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.

use crate::i18n::Translations;

pub fn translations() -> Translations {
    Translations {
        game_title: "Rustle",
        win_messages: crate::helpers::feedback::win::WIN_MESSAGES,
        game_copied: "Game copied to clipboard",
        not_enough_letters: "Not enough letters",
        word_not_found: "Word not found",
        hard_mode_alert: "Hard Mode can only be enabled at the start!",
        enter: "Enter",
        delete: "Delete",
        statistics: "Statistics",
        guess_distribution: "Guess Distribution",
        new_word: "New word in",
        share: "Share",
        share_failure: "Unable to share the results. This feature is available only in secure contexts (HTTPS), in some or all supporting browsers.",
        transfer: "Transfer",
        transfer_desc: "Click here to transfer your statistics to a new device.",
        total_tries: "Total tries",
        success_rate: "Success rate",
        current_streak: "Current streak",
        best_streak: "Best streak",
        discourage_browser: "You are using an embedded browser and may experience problems sharing or saving your results. We encourage you rather to use your device's default browser.",
        datepicker_title: "Choose a past date",
        datepicker_choose: "Choose",
        logout: "Logout",
    }
}
