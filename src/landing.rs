use dioxus::prelude::*;

use csv::StringRecord;
use std::f32::consts::PI;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct GlobalStats {
    count: usize,
    passed: usize,
    failed: usize,
    skip: usize,
    flake: usize,
    crash: usize,
}

fn percentage(count: usize, total: usize) -> f32 {
    (count as f32 * 100.0) / total as f32
}

#[component]
pub fn Landing() -> Element {
    let result = use_context::<Vec<StringRecord>>();

    let global_stats = result.iter().fold(GlobalStats::default(), |acc, record| {
        let mut new_acc = acc;
        match &record[1] {
            "Pass" => new_acc.passed += 1,
            "Skip" => new_acc.skip += 1,
            "Fail" => new_acc.failed += 1,
            "Flake" => new_acc.flake += 1,
            "Crash" => new_acc.crash += 1,
            _ => {},
        }
        new_acc.count = new_acc.passed + new_acc.failed + new_acc.skip + new_acc.flake + new_acc.crash;
        new_acc
    });

    rsx! {
        div {
            class: "flex flex-col space-y-4 rounded-3xl p-4 w-full h-fit shadow-xl shadow-slate-950",
            style: "background: linear-gradient(145deg, #020617 0, #02081f 60%, #020617 100%);",
            div { class: "border-1 border-slate-400 bg-slate-400/15 text-slate-400 w-fit rounded-3xl p-1 flex flex-row space-x-1 items-center",
                div { class: "bg-[#22c55e] rounded-full size-3" }
                p { class: "text-xs",
                    "Count: {global_stats.count} tests"
                }
            }
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4",
                StatCard {
                    name: "PASSED".to_string(),
                    color: "#22c55e".to_string(),
                    count: global_stats.passed,
                    stat: percentage(global_stats.passed, global_stats.count),
                }
                StatCard {
                    name: "FAILED".to_string(),
                    color: "#ff6467".to_string(),
                    count: global_stats.failed,
                    stat: percentage(global_stats.failed, global_stats.count),
                }
                StatCard {
                    name: "SKIPPED".to_string(),
                    color: "#ffdf20".to_string(),
                    count: global_stats.skip,
                    stat: percentage(global_stats.skip, global_stats.count),
                }
                StatCard {
                    name: "FLAKE".to_string(),
                    color: "#38bdf8".to_string(),
                    count: global_stats.flake,
                    stat: percentage(global_stats.flake, global_stats.count),
                }
                StatCard {
                    name: "CRASH".to_string(),
                    color: "#e7000b".to_string(),
                    count: global_stats.crash,
                    stat: percentage(global_stats.crash, global_stats.count),
                }
            }
            div { class: "mx-auto size-[200px]",
                StatsPieChart { stats: global_stats }
            }
            div { class: "mt-12 w-full bg-gray-900 overflow-hidden border-1 border-slate-700 rounded-lg",
                table { class: "w-full border-collapse border-spacing-0",
                    tr {
                        class: "border-b-1 border-slate-700",
                        style: "background: radial-gradient(circle at top, rgba(56, 189, 248, 0.1), rgba(15, 23, 42, 1));",
                        th { class: "text-left uppercase bold whitespace-nowrap py-2 px-3",
                            "Test name",
                        }
                        th { class: "text-left uppercase bold whitespace-nowrap py-2 px-3",
                            "Status",
                        }
                    }
                    for test in result[0..100].iter() {
                        tr { class: "text-sm hover:bg-[#38bef7]/5",
                            td { class: "py-2 px-3",
                                "{&test[0]}"
                            }
                            td { class: "py-2 px-3",
                                "{&test[1]}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(
    name: String,
    color: String,
    count: usize,
    stat: f32,
) -> Element {
    rsx! {
        div { class: "rounded-2xl p-4 border-1 border-slate-800 shadow-xl shadow-[#02081f] w-full h-fit bg-[#090f21] flex flex-col space-y-2",
            div { class: "flex flex-row space-x-2 flex items-center",
                div {
                    class: "rounded-full size-4",
                    style: format!("background-color: {color};"),
                }
                h3 { class: "text-sm text-gray-300",
                    "{name}"
                }
            }
            h2 {
                class: "text-2xl font-bold",
                style: format!("color: {color};"),
                "{count}",
            }
            p { class: "text-xs text-gray-400",
                "{stat:.1}% of total"
            }
        }
    }
}

struct Segment {
    percentage: f32,
    start: f32,
    end: f32,
    color: String,
}

#[component]
fn StatsPieChart(stats: GlobalStats) -> Element {
    let total = stats.count as f32;

    if total == 0.0 {
        return rsx!{};
    }

    let passed = stats.passed as f32 / total;
    let failed = stats.failed as f32 / total;
    let skip = stats.skip as f32 / total;
    let flake = stats.flake as f32 / total;
    let crash = stats.crash as f32 / total;

    let colors = (
        "#22c55e", // passed
        "#ff6467", // failed
        "#ffdf20", // skipped
        "#38bdf8", // flake
        "#e7000b", // crash
    );

    let mut segments: Vec<Segment> = Vec::new();
    let mut cumulative = 0.0_f32;

    for (pct, color) in [
        (passed, colors.0),
        (failed, colors.1),
        (skip, colors.2),
        (flake, colors.3),
        (crash, colors.4),
    ] {
        if pct > 0.0 {
            segments.push(Segment {
                percentage: pct * 100.0,
                start: cumulative,
                end: cumulative + pct,
                color: color.to_string(),
            });
            cumulative += pct;
        }
    }

    let radius: f32 = 80.0;
    let cx: f32 = 100.0;
    let cy: f32 = 100.0;

    let paths = segments
        .iter()
        .enumerate()
        .map(|(idx, seg)| {
            let start_angle = seg.start * 2.0 * PI;
            let end_angle = seg.end * 2.0 * PI;

            let x1 = cx + radius * start_angle.cos();
            let y1 = cy + radius * start_angle.sin();
            let x2 = cx + radius * end_angle.cos();
            let y2 = cy + radius * end_angle.sin();

            let large_arc_flag = if (end_angle - start_angle) > PI { 1 } else { 0 };

            let d = format!(
                "M {cx} {cy} L {x1} {y1} A {radius} {radius} 0 {large_arc_flag} 1 {x2} {y2} Z"
            );

            rsx! {
                path {
                    key: "{idx}",
                    d: "{d}",
                    fill: "{seg.color}",
                    opacity: "0.9",
                    stroke: "rgba(255, 255, 255, 0.1)",
                    "stroke-width": "1",
                }
            }
        });

    rsx! {
        svg {
            class: "max-w-[200px] h-auto",
            view_box: "0 0 200 200",
            {paths}
        }
    }
}
