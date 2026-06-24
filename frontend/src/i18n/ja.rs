// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.

use crate::i18n::Translations;

pub fn translations() -> Translations {
    Translations {
        game_title: "Rustle",
        win_messages: &["素晴らしい！", "見事", "よくできました！"],
        game_copied: "結果をクリップボードにコピーしました",
        not_enough_letters: "文字数が足りません",
        word_not_found: "単語が見つかりません",
        hard_mode_alert: "ハードモードは開始時にのみ有効にできます！",
        enter: "決定",
        delete: "削除",
        statistics: "統計情報",
        guess_distribution: "予測分布",
        new_word: "次の単語まで",
        share: "共有",
        share_failure: "結果を共有できません。この機能は安全なコンテキスト (HTTPS) でのみ利用可能です。",
        transfer: "データ移行",
        transfer_desc: "ここをクリックして、新しいデバイスに統計情報を移行します。",
        total_tries: "総挑戦回数",
        success_rate: "勝率",
        current_streak: "現在のストリーク",
        best_streak: "最大ストリーク",
        discourage_browser: "内蔵ブラウザを使用しているため、共有や保存で問題が発生する可能性があります。標準ブラウザの使用をお勧めします。",
        datepicker_title: "過去の日付を選択",
        datepicker_choose: "選択",
        logout: "ログアウト",
    }
}
