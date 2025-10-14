use eframe::egui;

pub fn draw_about_dialog(ctx: &egui::Context, show_about: &mut bool) {
    egui::Window::new("About")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_width(500.0);
            ui.heading(egui::RichText::new("Holding Trainer").size(18.0));
            ui.add_space(12.0);

            ui.label("Hi! I'm Jonathan Gañán Balboa, a cadet pilot currently in training.");
            ui.add_space(6.0);
            ui.label("During my instrument training, I found holding pattern entries challenging");
            ui.label("to visualize and practice efficiently. I needed a tool that could help me");
            ui.label("understand the three entry types (Direct, Teardrop, and Parallel) without");
            ui.label("always being connected to X-Plane.");
            ui.add_space(10.0);

            ui.label("So I built this tool to practice more effectively and improve my skills.");
            ui.label("It helped me tremendously, and I hope it helps other aviation students too!");
            ui.add_space(12.0);

            ui.separator();
            ui.add_space(10.0);

            ui.label(egui::RichText::new("Features:").strong().size(14.0));
            ui.add_space(5.0);
            ui.label("  • Simulate Mode: Practice with any VOR worldwide, no sim needed");
            ui.label("  • X-Plane 11 Mode: Real-time practice with your favorite aircraft");
            ui.label("  • Visual Sectors: See entry zones and get instant feedback");
            ui.label("  • Interactive: Adjust heading and see results immediately");
            ui.add_space(12.0);

            ui.separator();
            ui.add_space(10.0);

            ui.label(egui::RichText::new("If you find this tool useful, please consider supporting").size(12.0));
            ui.label(egui::RichText::new("the development so I can keep improving it!").size(12.0));
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Close").clicked() {
                    *show_about = false;
                }

                ui.add_space(20.0);

                let kofi_button = egui::Button::new(
                    egui::RichText::new("☕ Support on Ko-fi")
                        .size(13.0)
                        .color(egui::Color32::WHITE)
                ).fill(egui::Color32::from_rgb(255, 95, 95));

                if ui.add_sized([140.0, 28.0], kofi_button).clicked() {
                    let _ = open::that("https://ko-fi.com/jgananb");
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(6.0);
            ui.label(egui::RichText::new("Made with ❤ by Jonathan Gañán Balboa • 2025").size(10.0).italics().color(egui::Color32::from_rgb(120, 120, 120)));
        });
}

pub fn draw_how_to_fly_dialog(ctx: &egui::Context, show_how_it_works: &mut bool) {
    egui::Window::new("How to Fly Holding Patterns")
        .collapsible(false)
        .resizable(true)
        .default_width(600.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading(egui::RichText::new("HOLDING PATTERN PROCEDURES").size(18.0).strong());
                ui.add_space(10.0);

                ui.label("A holding pattern is a predetermined maneuver used to keep an aircraft within");
                ui.label("a specified airspace while awaiting further clearance from ATC.");
                ui.add_space(12.0);

                ui.separator();
                ui.add_space(10.0);

                ui.heading(egui::RichText::new("ELEMENTS OF A HOLDING PATTERN").size(15.0));
                ui.add_space(8.0);
                ui.label("• Fix: The geographical position that serves as the reference point");
                ui.label("• Inbound Leg: The track toward the fix (1 min below 14,000 ft MSL)");
                ui.label("• Outbound Leg: The track away from the fix");
                ui.label("• Standard Pattern: Right turns (most common)");
                ui.label("• Non-Standard Pattern: Left turns (when published)");
                ui.add_space(12.0);

                ui.separator();
                ui.add_space(10.0);

                ui.heading(egui::RichText::new("DETERMINING YOUR ENTRY").size(15.0));
                ui.add_space(8.0);
                ui.label("The FAA recommends three entry procedures based on your approach angle");
                ui.label("relative to the holding course. These entries were designed to keep aircraft");
                ui.label("within protected airspace during the entry maneuver.");
                ui.add_space(10.0);

                ui.strong("SECTOR BOUNDARIES (Standard Right Turns):");
                ui.label("  • DIRECT Entry: 180° sector (270° to 90° relative to inbound)");
                ui.label("  • TEARDROP Entry: 70° sector (90° to 160° relative to inbound)");
                ui.label("  • PARALLEL Entry: 110° sector (160° to 270° relative to inbound)");
                ui.add_space(8.0);

                ui.strong("SECTOR BOUNDARIES (Non-Standard Left Turns):");
                ui.label("  • DIRECT Entry: 180° sector (270° to 90° relative to inbound)");
                ui.label("  • PARALLEL Entry: 110° sector (90° to 200° relative to inbound)");
                ui.label("  • TEARDROP Entry: 70° sector (200° to 270° relative to inbound)");
                ui.add_space(8.0);

                ui.label(egui::RichText::new("💡 TIP: If within 10° of a boundary, you may use either entry method.").italics().color(egui::Color32::from_rgb(100, 200, 255)));
                ui.add_space(12.0);

                ui.separator();
                ui.add_space(10.0);

                ui.heading(egui::RichText::new("THE THREE ENTRY PROCEDURES").size(15.0));
                ui.add_space(10.0);

                ui.strong("1. DIRECT ENTRY (180° Sector)");
                ui.label("Use when approaching from the holding side:");
                ui.label("  1) Cross the fix");
                ui.label("  2) Turn immediately to the outbound heading");
                ui.label("  3) Fly outbound for 1 minute (or 1.5 min above 14,000')");
                ui.label("  4) Turn inbound and proceed with the hold");
                ui.label("This is the most common entry, covering half the compass rose.");
                ui.add_space(10.0);

                ui.strong("2. TEARDROP ENTRY (70° Sector)");
                ui.label("Use when approaching from the offset side:");
                ui.label("  1) Cross the fix");
                ui.label("  2) Turn 30° outbound from the inbound course (toward holding side)");
                ui.label("  3) Fly for 1 minute on this heading");
                ui.label("  4) Turn inbound to intercept the inbound course (>180° turn)");
                ui.label("  5) Proceed inbound to the fix and continue the hold");
                ui.label("The teardrop keeps you on the protected side while setting up for the hold.");
                ui.add_space(10.0);

                ui.strong("3. PARALLEL ENTRY (110° Sector)");
                ui.label("Use when approaching from the non-holding side:");
                ui.label("  1) Cross the fix");
                ui.label("  2) Turn to parallel the inbound course outbound (opposite direction)");
                ui.label("  3) Fly for 1 minute on the parallel track");
                ui.label("  4) Turn to intercept and re-cross the fix");
                ui.label("  5) Turn outbound and proceed with the standard hold");
                ui.label("This entry ensures you stay within protected airspace from the opposite side.");
                ui.add_space(12.0);

                ui.separator();
                ui.add_space(10.0);

                ui.heading(egui::RichText::new("TIMING & SPEEDS").size(15.0));
                ui.add_space(8.0);
                ui.label("• Below 14,000' MSL: 1-minute legs");
                ui.label("• Above 14,000' MSL: 1.5-minute legs");
                ui.label("• Timing starts when abeam the fix outbound, or wings level, whichever occurs later");
                ui.add_space(8.0);
                ui.strong("Maximum Holding Speeds:");
                ui.label("  • Below 6,000' MSL: 200 KIAS");
                ui.label("  • 6,001' - 14,000' MSL: 230 KIAS");
                ui.label("  • Above 14,000' MSL: 265 KIAS");
                ui.add_space(12.0);

                ui.separator();
                ui.add_space(10.0);

                ui.heading(egui::RichText::new("WIND CORRECTION").size(15.0));
                ui.add_space(8.0);
                ui.label("Adjust your heading to compensate for wind drift:");
                ui.label("• Determine wind correction angle on the inbound leg");
                ui.label("• On the outbound leg, triple the inbound correction");
                ui.label("• Adjust timing to achieve 1-minute inbound leg");
                ui.label("• If early returning to fix: increase outbound time");
                ui.label("• If late returning to fix: decrease outbound time");
                ui.add_space(12.0);

                ui.separator();
                ui.add_space(10.0);

                ui.heading(egui::RichText::new("THE FIVE T's - YOUR HOLDING CHECKLIST").size(15.0));
                ui.add_space(8.0);
                ui.label("Remember these Five T's at every fix passage:");
                ui.label("  1. TURN to the appropriate heading");
                ui.label("  2. TIME - start your stopwatch");
                ui.label("  3. TWIST - set your OBS/CDI as needed");
                ui.label("  4. THROTTLE - adjust power for holding speed");
                ui.label("  5. TALK - report to ATC (if required)");
                ui.add_space(12.0);

                ui.separator();
                ui.add_space(10.0);

                ui.heading(egui::RichText::new("PROFESSIONAL TIPS").size(15.0));
                ui.add_space(8.0);
                ui.label("✓ Reduce speed 3 minutes before reaching the fix");
                ui.label("✓ Use standard rate turns (3°/second or 30° bank, whichever is less)");
                ui.label("✓ Always confirm holding instructions with ATC");
                ui.label("✓ Practice the 'thumb method' to quickly determine entry type");
                ui.label("✓ Stay ahead of the aircraft - plan your entry early");
                ui.label("✓ When in doubt, the direct entry is acceptable from any angle");
                ui.label("✓ Maintain situational awareness of protected vs non-protected airspace");
                ui.add_space(15.0);

                if ui.button("Close").clicked() {
                    *show_how_it_works = false;
                }
            });
        });
}
