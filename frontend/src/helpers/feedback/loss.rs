// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Loss taunt quotes.

/// Selects a randomized loss taunt.
pub fn get_loss_taunt(solution: &str) -> String {
    let taunts = &[
        format!("Better luck next time! It was {}.", solution),
        format!("Oof, that was close. It was {}.", solution),
        format!("No guesses left! It was {}.", solution),
        format!("Skill issue. The word was {}.", solution),
        format!("Unfortunate. The word was {}.", solution),
        format!("Outplayed by a dictionary! It was {}.", solution),
        format!("Try four-letter words? It was {}.", solution),
        format!("The dictionary wins! It was {}.", solution),
        format!("System overload! It was {}.", solution),
        format!("Close, but no cigar. It was {}.", solution),
        format!("The word won. It was {}.", solution),
        format!("Spell it backwards next time? It was {}.", solution),
        format!("Better luck next round. It was {}.", solution),
        format!("Stiff luck! The word was {}.", solution),
        format!("Fate had other plans. It was {}.", solution),
        format!("A dictionary crushed you. It was {}.", solution),
        format!("Task failed. The word was {}.", solution),
        format!("Close, but no stats. It was {}.", solution),
        format!("Streak ended! The word was {}.", solution),
        format!("Insert coin to try again. It was {}.", solution),
    ];
    let idx = (js_sys::Math::random() * taunts.len() as f64).floor() as usize;
    taunts[idx].clone()
}
