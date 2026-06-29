// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Unified game play feedback module.

pub mod intermediate;
pub mod loss;
pub mod win;

pub use intermediate::get_intermediate_comment;
pub use loss::get_loss_taunt;
pub use win::WIN_MESSAGES;
