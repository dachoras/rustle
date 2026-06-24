// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.

use crate::i18n::Translations;

pub fn translations() -> Translations {
    Translations {
        game_title: "Rustle",
        win_messages: &["Отличная работа!", "Потрясающе", "Молодец!"],
        game_copied: "Игра скопирована в буфер обмена",
        not_enough_letters: "Недостаточно букв",
        word_not_found: "Слово не найдено",
        hard_mode_alert: "Сложный режим можно включить только в начале игры!",
        enter: "Ввод",
        delete: "Стереть",
        statistics: "Статистика",
        guess_distribution: "Распределение попыток",
        new_word: "Новое слово через",
        share: "Поделиться",
        share_failure: "Не удалось поделиться результатами. Эта функция доступна только в безопасном контексте (HTTPS).",
        transfer: "Перенос",
        transfer_desc: "Нажмите здесь, чтобы перенести статистику на новое устройство.",
        total_tries: "Всего попыток",
        success_rate: "Процент побед",
        current_streak: "Текущая серия",
        best_streak: "Лучшая серия",
        discourage_browser: "Вы используете встроенный браузер и можете столкнуться с проблемами. Мы рекомендуем использовать стандартный браузер.",
        datepicker_title: "Выберите дату в прошлом",
        datepicker_choose: "Выбрать",
        logout: "Выйти",
    }
}
