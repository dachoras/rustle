// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Intermediate feedback quotes.

/// Selects a randomized intermediate feedback comment based on current theme and match performance.
pub fn get_intermediate_comment(theme: &str, total_matches: usize) -> String {
    let list: &[&str] = match theme {
        "christmas" => {
            if total_matches == 0 {
                &["Coal guess!", "Grinch energy.", "Santa shook his head."]
            } else if total_matches <= 2 {
                &["Tiny present.", "Tinsel effort.", "Stocking stuffer."]
            } else {
                &["Nice list guess!", "Jingle bells!", "Holiday magic!"]
            }
        }
        "halloween" => {
            if total_matches == 0 {
                &["Ghosted!", "Spooked!", "Ghouls laughed."]
            } else if total_matches <= 2 {
                &["Tiny treat.", "Pumpkin brain.", "Skeleton guess."]
            } else {
                &["Frighteningly good!", "Witchcraft!", "Graveyard smash!"]
            }
        }
        "easter" => {
            if total_matches == 0 {
                &["Empty basket!", "Egg broke!", "No eggs here."]
            } else if total_matches <= 2 {
                &["Small egg found.", "Average hop.", "Pastel attempt."]
            } else {
                &["Egg-cellent!", "Almost there!", "Golden Egg close!"]
            }
        }
        "thanksgiving" => {
            if total_matches == 0 {
                &["Cold leftovers.", "Gravy went cold.", "Turkey escaped."]
            } else if total_matches <= 2 {
                &["Appetizer guess.", "Pass the gravy.", "Small slice of pie."]
            } else {
                &["Gourmet guess!", "Stuffed with letters!", "Feast mode!"]
            }
        }
        "newyear" => {
            if total_matches == 0 {
                &[
                    "Resolution failed!",
                    "Dropped the ball.",
                    "Countdown fizzled.",
                ]
            } else if total_matches <= 2 {
                &["Counting down.", "Sparkler guess.", "Sober start."]
            } else {
                &[
                    "Midnight is close!",
                    "Fireworks ready!",
                    "Winning streak start!",
                ]
            }
        }
        "valentine" => {
            if total_matches == 0 {
                &["Heartbroken!", "Unrequited love.", "Swiped left."]
            } else if total_matches <= 2 {
                &["Secret admirer?", "Sent a card.", "Friendzoned."]
            } else {
                &[
                    "Cupid approved!",
                    "Heart rate rising!",
                    "Perfect match close!",
                ]
            }
        }
        "independence" => {
            if total_matches == 0 {
                &["Fireworks fizzled!", "A total dud.", "No liberty here."]
            } else if total_matches <= 2 {
                &["Small sparkler.", "Smoke, no fire.", "Parade guess."]
            } else {
                &[
                    "Spectacular spark!",
                    "Revolutionary!",
                    "Red, white, correct!",
                ]
            }
        }
        "stpatrick" => {
            if total_matches == 0 {
                &["Out of luck!", "No pot of gold.", "Leprechaun pinch."]
            } else if total_matches <= 2 {
                &["Three-leaf clover.", "Sip of cider.", "Pinch protected."]
            } else {
                &["Gold is close!", "Irish luck active!", "Four-leaf clover!"]
            }
        }
        _ => {
            if total_matches == 0 {
                &[
                    "Oof, absolute zero.",
                    "Playing blindfolded?",
                    "Swing and a miss.",
                    "Keyboard unplugged?",
                    "Not a single letter.",
                    "Zilch. Nada. Nothing.",
                    "Clean slate.",
                    "Random letter generator?",
                    "No letters harmed.",
                    "Luck is buffering.",
                    "Draft version guess?",
                    "Broken clock guess.",
                    "Mind-clearing guess?",
                    "Bold strategy.",
                    "Success not found.",
                    "Playing blind?",
                    "Completely off mark.",
                    "Zero signal.",
                    "Empty scan.",
                    "Blank result.",
                ]
            } else if total_matches <= 2 {
                &[
                    "Not terrible.",
                    "At least not zero!",
                    "Small step.",
                    "Warming up.",
                    "Slight progress.",
                    "Beginner tier guess.",
                    "Lukewarm result.",
                    "Found a piece.",
                    "Better than nothing.",
                    "Baseline established.",
                    "Mild spark.",
                    "Minor progress.",
                    "Signs of life.",
                    "Visiting neighborhood.",
                    "Toes dipped.",
                    "Lukewarm start.",
                    "One correct zone.",
                    "Slightly warm.",
                    "Calibrating.",
                    "On the map.",
                ]
            } else {
                &[
                    "Now we're cookin'!",
                    "Highly logical!",
                    "So close!",
                    "Great deduction!",
                    "Within reach!",
                    "Don't choke now.",
                    "Clean up details.",
                    "Chef's kiss guess!",
                    "Galaxy brain time.",
                    "Almost there!",
                    "Outstanding skills!",
                    "Recipe is ready.",
                    "Brilliant play.",
                    "Word is shaking.",
                    "Masterclass close.",
                    "Letters are waving.",
                    "Absolute precision.",
                    "Cross the line!",
                    "One final push!",
                    "Nearly perfect!",
                ]
            }
        }
    };
    let idx = (js_sys::Math::random() * list.len() as f64).floor() as usize;
    list[idx].to_string()
}
