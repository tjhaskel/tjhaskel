use std::time::Duration;

use simpleterm_profile::{art::*, text::*, terminal::Terminal};

fn main() {
    // Create a window and display the GEO art (from art.rs) for 2 seconds.
    let mut term: Terminal = Terminal::new("simpleterm test", (420, 120), DARK_GREY, EMERALD, "LeagueSpartan-Regular.ttf", 32);
    term.art_font_size = 24;
    let c_len: usize = COLORS.len();

    while term.active {
        let mut count: usize = 0;
        for i in 0..16 {
            for _ in 0..4 {
                term.fg_color = COLORS[count % c_len];
                term.display_art(DANCES[i], Duration::from_millis(200));
                count += 1;
            }
        }

        term.tell("Welcome to Trevan's GitHub!");
    }
}
