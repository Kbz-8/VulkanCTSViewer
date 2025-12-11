use dioxus::prelude::*;

use std::{collections::HashMap, thread::current};
use std::str::FromStr;

use csv::StringRecord;
use std::f32::consts::PI;

use strum::{EnumCount, IntoEnumIterator};

use crate::components::select::*;

const PAGE_SIZE: usize = 100_usize;

#[derive(
    Debug,
    Eq,
    Hash,
    Clone,
    Copy,
    PartialEq,
    strum::EnumCount,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
)]
enum TestStatus {
    Pass,
    Fail,
    Skip,
    Flake,
    Crash,
}

impl TestStatus {
    const fn emoji(&self) -> &'static str {
        match self {
            TestStatus::Pass => "✅",
            TestStatus::Fail => "❌",
            TestStatus::Skip => "❎",
            TestStatus::Flake => "⚠️",
            TestStatus::Crash => "💥",
        }
    }

    const fn color(&self) -> &'static str {
        match self {
            TestStatus::Pass => "#22c55e",
            TestStatus::Fail => "#ff6467",
            TestStatus::Skip => "#ffdf20",
            TestStatus::Flake => "#38bdf8",
            TestStatus::Crash => "#e7000b",
        }
    }
}

fn percentage(count: usize, total: f32) -> f32 {
    (count as f32 * 100.0) / total
}

#[component]
pub fn Landing() -> Element {
    let result = use_context::<Signal<Vec<StringRecord>>>();

    let global_stats = use_memo(move || result
        .read()
        .iter()
        .fold(HashMap::from_iter(TestStatus::iter().map(|s| (s, 0))), |mut acc, record| {
            if let Ok(status) = TestStatus::from_str(&record[1]) {
                *acc.entry(status).or_insert(0) += 1;
            }
            acc
        })
    );

    let total = use_memo(move || global_stats.read().iter().fold(0.0_f32, |acc, (_, v)| acc + *v as f32));

    let stats_cards = TestStatus::iter().map(|s| {
        rsx! {
            StatCard {
                name: s.to_string(),
                color: s.color().to_string(),
                count: global_stats.read()[&s],
                stat: percentage(global_stats.read()[&s], total()),
            }
        }
    });

    let mut filter: Signal<Option<TestStatus>> = use_signal(|| None);
    let mut filtered_count = use_signal(|| 0_usize);

    let mut current_page = use_memo(move || 0_usize);
    let mut page_count = use_memo(move || filtered_count().max(PAGE_SIZE - 1) / PAGE_SIZE);

    let page = use_memo(move || {
        if let Some(f) = filter() {
            current_page.set(0);
            let mut count = 0_usize;
            filtered_count.set(0_usize);
            let shift = (current_page() * PAGE_SIZE);
            result
                .read()
                .iter()
                .filter(|r| {
                    if let Ok(status) = TestStatus::from_str(&r[1]) && status == f {
                        filtered_count += 1;
                        if *filtered_count.peek() < shift {
                            return false;
                        }
                        count += 1;
                        if count >= PAGE_SIZE {
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                })
                .cloned()
                .collect()
        } else {
            filtered_count.set(total() as usize);
            let start = current_page() * PAGE_SIZE;
            let end = result.read().len().min((current_page() * PAGE_SIZE) + PAGE_SIZE);
            result.read()[start..end].to_vec()
        }
    });

    let statuses = TestStatus::iter().enumerate().map(|(i, s)| {
        rsx! {
            SelectOption::<Option<TestStatus>> {
                index: i,
                value: s,
                text_value: "{s}",
                {format!("{} {s}", s.emoji())}
                SelectItemIndicator {}
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-col space-y-4 rounded-3xl p-4 pt-8 w-full h-fit shadow-xl shadow-slate-950",
            style: "background: linear-gradient(145deg, #020617 0, #02081f 60%, #020617 100%);",
            div { class: "flex flex-row space-x-4",
                div { class: "border-1 border-[#38bdf8] bg-[#38bdf8]/15 text-slate-400 w-fit rounded-3xl py-1 px-2 flex flex-row space-x-1 items-center",
                    div { class: "bg-[#38bdf8] rounded-full size-3" }
                    p { class: "text-xs",
                        "Total: {total} tests"
                    }
                }
                div { class: "border-1 border-[#22c55e] bg-[#22c55e]/15 text-slate-400 w-fit rounded-3xl py-1 px-2 flex flex-row space-x-1 items-center",
                    div { class: "bg-[#22c55e] rounded-full size-3" }
                    p { class: "text-xs",
                        "Filtered: {filtered_count} tests"
                    }
                }
            }
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4",
                {stats_cards}
            }
            div { class: "mx-auto size-[200px]",
                StatsPieChart { stats: global_stats, total }
            }
            div { class: "mt-12 w-full flex flex-row justify-between text-gray-400 text-sm",
                p { "Page {current_page() + 1} of {page_count() + 1}" }
                div { class: "flex flex-row space-x-2",
                    button { class: "pagination-button",
                        disabled: current_page() == 0,
                        onclick: move |_| current_page.set(0_usize),
                        "First"
                    }
                    button { class: "pagination-button",
                        disabled: current_page() == 0,
                        onclick: move |_| current_page -= 1,
                        "Prev"
                    }

                    if current_page() > 2 {
                        p { "..." }
                    }

                    for i in ((current_page() as i32 - 2).max(0) as usize)..=(current_page() + 2).min(page_count()) {
                        button { class: "pagination-button",
                            "data-active": i == current_page(),
                            onclick: move |_| current_page.set(i),
                            "{i + 1}"
                        }
                    }

                    if current_page() < (page_count() as i32 - 2).max(0) as usize {
                        p { "..." }
                    }

                    button { class: "pagination-button",
                        disabled: current_page() >= page_count(),
                        onclick: move |_| current_page += 1,
                        "Next"
                    }
                    button { class: "pagination-button",
                        disabled: current_page() >= page_count(),
                        onclick: move |_| current_page.set(page_count()),
                        "Last"
                    }
                }
            }
            div { class: "w-full bg-gray-900 overflow-hidden border-1 border-slate-700 rounded-lg text-gray-400",
                table { class: "w-full border-collapse border-spacing-0",
                    tr {
                        class: "border-b-1 border-slate-700",
                        style: "background: radial-gradient(circle at top, rgba(56, 189, 248, 0.1), rgba(15, 23, 42, 1));",
                        th { class: "text-left uppercase bold whitespace-nowrap py-2 px-3",
                            "Test name",
                        }
                        th { class: "uppercase bold whitespace-nowrap py-2 px-3",
                            Select::<Option<TestStatus>> {
                                placeholder: "STATUS",
                                on_value_change: move |value: Option<Option<TestStatus>>| filter.set(value.unwrap_or(None)),
                                SelectTrigger {
                                    class: "select-trigger mx-auto w-fit !bg-transparent !shadow-none !text-gray-400 cursor-pointer uppercase",
                                    aria_label: "Select Trigger",
                                    SelectValue { class: "!bg-transparent !shadow-none !text-gray-400" }
                                }
                                SelectList {
                                    aria_label: "Select status",
                                    SelectGroup {
                                        {statuses}
                                        SelectOption::<Option<TestStatus>> {
                                            index: TestStatus::COUNT,
                                            value: None,
                                            text_value: "Status",
                                            "🔄 None"
                                            SelectItemIndicator {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    for test in page.iter() {
                        tr { class: "text-sm hover:bg-[#38bef7]/5",
                            td { class: "py-2 px-3",
                                "{&test[0]}"
                            }
                            td { class: "py-2 px-3",
                                StatusBadge {
                                    status: match TestStatus::from_str(&test[1]) {
                                        Ok(s) => Some(s),
                                        _ => None,
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(name: String, color: String, count: usize, stat: f32) -> Element {
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

#[component]
fn StatusBadge(status: Option<TestStatus>) -> Element {
    let color = match status {
        Some(s) => s.color(),
        None => "#FFF",
    };
    let name = match status {
        Some(s) => s.to_string(),
        None => "Unrecognized".to_string(),
    };

    rsx! {
        div {
            class: "mx-auto border-1 py-1 px-3 rounded-3xl text-xs w-fit select-none",
            style: format!(r#"
                background-color: {color}0F;
                color: {color};
                border-color: {color};
            "#),
            "{name}"
        }
    }
}

#[component]
fn StatsPieChart(stats: ReadSignal<HashMap<TestStatus, usize>>, total: ReadSignal<f32>) -> Element {
    struct Segment {
        percentage: f32,
        start: f32,
        end: f32,
        color: String,
    }

    if total() == 0.0 {
        return rsx! {};
    }

    let mut segments: Vec<Segment> = Vec::new();
    let mut cumulative = 0.0_f32;

    for (key, val) in stats.read().iter() {
        let stat = *val as f32 / total();
        if stat > 0.0 {
            segments.push(Segment {
                percentage: stat * 100.0,
                start: cumulative,
                end: cumulative + stat,
                color: key.color().to_string(),
            });
            cumulative += stat;
        }
    }

    let radius: f32 = 80.0;
    let cx: f32 = 100.0;
    let cy: f32 = 100.0;

    let paths = segments.iter().enumerate().map(|(idx, seg)| {
        let start_angle = seg.start * 2.0 * PI;
        let end_angle = seg.end * 2.0 * PI;

        let x1 = cx + radius * start_angle.cos();
        let y1 = cy + radius * start_angle.sin();
        let x2 = cx + radius * end_angle.cos();
        let y2 = cy + radius * end_angle.sin();

        let large_arc_flag = if (end_angle - start_angle) > PI { 1 } else { 0 };

        let d =
            format!("M {cx} {cy} L {x1} {y1} A {radius} {radius} 0 {large_arc_flag} 1 {x2} {y2} Z");

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
