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
                    "Ghouls laughed from the shadows.",
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
                    "The turkey escaped.",
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
                    "The Cupid algorithm swiped left.",
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
                    "Leprechaun pinched you.",
                ]
            } else if total_matches <= 2 {
                &[
                    "Found a three-leaf clover, but we need four!",
                    "A sip of cider, but we need more.",
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
                    "Oof, absolute zero. Are you playing blindfolded?",
                    "A swing and a complete miss.",
                    "Is your keyboard even plugged in?",
                    "Not a single letter. Impressively bad!",
                    "Error 404: Correct letters not found.",
                    "Are we guessing in a different language?",
                    "Nothing. Nada. Zilch. Rien. Niente.",
                    "You hit nothing but net... outside the gym.",
                    "We call that a clean slate.",
                    "Did you copy-paste this from a random generator?",
                    "No letters harmed in the making of this guess.",
                    "This is a word game, not a random letter selector.",
                    "Well, at least you eliminated five letters.",
                    "Zero matches. Your luck is currently buffering.",
                    "Is this a draft or did you actually mean to submit that?",
                    "Even a broken clock gets a match twice a day.",
                    "Let's pretend that guess was just to clear your mind.",
                    "Bold strategy. Let's see if it pays off.",
                    "Error: Success not found in this sector.",
                    "You're playing on hard mode, even if hard mode is off.",
                ]
            } else if total_matches <= 2 {
                &[
                    "Not terrible, but let's not write home about it.",
                    "At least it's not zero!",
                    "A small step, but we need a giant leap.",
                    "You're warmed up, now make a real guess.",
                    "Slight progress detected. Keep going.",
                    "It's a start, but don't order the trophy just yet.",
                    "One small step for a guesser, one giant search for a word.",
                    "Getting warmer! Just like a lukewarm cup of tea.",
                    "You found a piece of the puzzle. Now find the rest.",
                    "Better than nothing, but we have high standards.",
                    "Okay, we've got a baseline. Let's build on it.",
                    "A mild spark. Let's turn it into a fire.",
                    "Progress is progress, no matter how small.",
                    "You've unlocked the beginner tier of this word.",
                    "We have signs of life!",
                    "At least your letters are visiting the neighborhood.",
                    "Not bad, but the dictionary is still winning.",
                    "You've dipped your toes in. Ready to jump in?",
                    "A solid foundation, if the building was one story.",
                    "Keep calibrating, you're on the map.",
                ]
            } else {
                &[
                    "Now we're cookin'!",
                    "Highly logical guess!",
                    "So close you can almost taste the victory.",
                    "Great deduction! You're on fire.",
                    "Victory is within reach!",
                    "Almost perfect! Don't choke now.",
                    "You've basically got it. Just clean up the details.",
                    "Chef's kiss guess! Extremely close.",
                    "You're playing chess while the dictionary plays checkers.",
                    "Now that is what I call a galaxy brain guess.",
                    "Almost there! Just a tiny adjustment needed.",
                    "Outstanding deduction skills!",
                    "You have the recipe. Now bake the cake.",
                    "Brilliant line of play.",
                    "The word is shaking in its boots.",
                    "A masterclass in process of elimination.",
                    "You're so close, the letters are practically waving.",
                    "Absolute precision. Finish the job!",
                    "You've got the map, now cross the finish line.",
                    "Outstanding. One final push!",
                ]
            }
        }
    };
    let idx = (js_sys::Math::random() * list.len() as f64).floor() as usize;
    list[idx].to_string()
}
