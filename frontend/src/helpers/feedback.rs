// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Game play intermediate feedback comments.

/// Selects a randomized intermediate feedback comment based on current theme and match performance.
pub fn get_intermediate_feedback(theme: &str, total_matches: usize) -> String {
    let list: &[&str] = match theme {
        "christmas" => {
            if total_matches == 0 {
                &[
                    "You got coal on that guess!",
                    "Grinch energy detected.",
                    "Santa is shaking his head.",
                ]
            } else if total_matches <= 2 {
                &[
                    "A tiny present, but we want the big one!",
                    "Tinsel level effort.",
                    "Stocking stuffer guess.",
                ]
            } else {
                &[
                    "Santa is checking his list... nice guess!",
                    "Jingle bells are ringing!",
                    "Pure holiday magic!",
                ]
            }
        }
        "halloween" => {
            if total_matches == 0 {
                &[
                    "Ghosted! Absolute zero.",
                    "Spooked by that guess!",
                    "Ridley laughed from the shadows.",
                ]
            } else if total_matches <= 2 {
                &[
                    "A tiny treat, keep digging.",
                    "Pumpkin level brain power.",
                    "A skeleton of a guess.",
                ]
            } else {
                &[
                    "Frighteningly good guess!",
                    "Witchcraft! You're so close.",
                    "A graveyard smash!",
                ]
            }
        }
        "easter" => {
            if total_matches == 0 {
                &[
                    "Easter basket is empty!",
                    "Egg broke!",
                    "No eggs found in this bush.",
                ]
            } else if total_matches <= 2 {
                &[
                    "Found a small egg, keep hopping.",
                    "Average bunny hop.",
                    "A pastel attempt.",
                ]
            } else {
                &[
                    "Golden Egg is within reach!",
                    "Hop, hop! You're almost there!",
                    "Egg-ceptional hunting skills!",
                ]
            }
        }
        "thanksgiving" => {
            if total_matches == 0 {
                &[
                    "Cold leftovers. 0 matches.",
                    "The gravy went cold.",
                    "Ridley stole the stuffing.",
                ]
            } else if total_matches <= 2 {
                &[
                    "A small slice of pie, but we want the feast!",
                    "Appetizer guess.",
                    "Pass the cranberry sauce.",
                ]
            } else {
                &[
                    "Now that's a gourmet guess!",
                    "Stuffed with correct letters!",
                    "Feast mode activated!",
                ]
            }
        }
        "newyear" => {
            if total_matches == 0 {
                &[
                    "Resolution failed already! 0 matches.",
                    "Dropped the ball early.",
                    "Countdown fizzled out.",
                ]
            } else if total_matches <= 2 {
                &[
                    "Counting down... keep trying.",
                    "Sparkler level attempt.",
                    "A sober start.",
                ]
            } else {
                &[
                    "Midnight is close! Great guess!",
                    "Fireworks are about to explode!",
                    "New Year, new winning streak!",
                ]
            }
        }
        "valentine" => {
            if total_matches == 0 {
                &[
                    "Heartbroken! 0 matches.",
                    "Unrequited love.",
                    "Ridley swiped left.",
                ]
            } else if total_matches <= 2 {
                &[
                    "A secret admirer? 1-2 matches.",
                    "Sent a card, waiting for reply.",
                    "Friendzoned guess.",
                ]
            } else {
                &[
                    "Cupid approved! Extremely close.",
                    "Heart rate rising!",
                    "A match made in heaven!",
                ]
            }
        }
        "independence" => {
            if total_matches == 0 {
                &[
                    "Fireworks fizzled! 0 matches.",
                    "A total dud.",
                    "No liberty in this guess.",
                ]
            } else if total_matches <= 2 {
                &[
                    "A small sparkler, let's get a big explosion!",
                    "Smoke but no fire.",
                    "Parade level effort.",
                ]
            } else {
                &[
                    "Spectacular fireworks! Almost there!",
                    "A revolutionary guess!",
                    "Red, white, and correct!",
                ]
            }
        }
        "stpatrick" => {
            if total_matches == 0 {
                &[
                    "Out of luck! 0 matches.",
                    "No gold at the end of this rainbow.",
                    "Ridley pinched you.",
                ]
            } else if total_matches <= 2 {
                &[
                    "Found a three-leaf clover, but we need four!",
                    "A sip of Guinness, but we need more.",
                    "Pinch protection active.",
                ]
            } else {
                &[
                    "Pot of gold is right around the corner!",
                    "Irish luck is on your side!",
                    "Four-leaf clover detected!",
                ]
            }
        }
        _ => {
            if total_matches == 0 {
                &[
                    "Absolute zero. Even Ridley felt that.",
                    "Are you playing blindfolded?",
                    "0 matches. Time to check visor settings.",
                    "A swing and a complete miss.",
                    "No signal. Is your keyboard plugged in?",
                ]
            } else if total_matches <= 2 {
                &[
                    "Not terrible, but Ridley is still laughing.",
                    "Scanning... low energy detected.",
                    "A minor upgrade, but we need more power.",
                    "At least it's not zero!",
                    "Keep scanning, Space Hunter.",
                ]
            } else {
                &[
                    "Now we're cookin' with plasma!",
                    "Energy tank refilled!",
                    "Samus is nodding in approval.",
                    "Calculations: highly logical.",
                    "Visor scanning reveals... hope!",
                ]
            }
        }
    };
    let idx = (js_sys::Math::random() * list.len() as f64).floor() as usize;
    list[idx].to_string()
}
