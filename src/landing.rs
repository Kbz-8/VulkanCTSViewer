use crate::components::{select::*, skeleton::*};
use crate::loader::{Loader, Suspense};
use csv::{ReaderBuilder, StringRecord};
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use dioxus_sdk_time::*;
use std::f32::consts::PI;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;
use std::{collections::HashMap, thread::current};
use strum::{EnumCount, IntoEnumIterator};

const PAGE_SIZE: usize = 100_usize;

// Wrapper for displaying a duration in h:m:s (integer seconds, rounded down)
struct HMSDuration(Duration);
impl fmt::Display for HMSDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ms = self.0.as_millis();
        let hours = ms / 3_600_000;
        ms %= 3_600_000;
        let mins = ms / 60_000;
        ms %= 60_000;
        let secs = ms / 1000;
        ms %= 1000;

        if hours > 0 {
            write!(f, "{}:{:02}:{:02}.{}", hours, mins, secs, ms)
        } else if mins > 0 {
            write!(f, "0:{}:{:02}.{}", mins, secs, ms)
        } else {
            write!(f, "0:0:{}.{}", secs, ms)
        }
    }
}

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
    Warn,
    Skip,
    Crash,
    Timeout,
}

impl TestStatus {
    const fn emoji(&self) -> &'static str {
        match self {
            TestStatus::Pass => "✅",
            TestStatus::Fail => "❌",
            TestStatus::Skip => "❎",
            TestStatus::Timeout => "⏱️",
            TestStatus::Warn => "⚠️",
            TestStatus::Crash => "💥",
        }
    }

    const fn color(&self) -> &'static str {
        match self {
            TestStatus::Pass => "#22c55e",
            TestStatus::Fail => "#ff6467",
            TestStatus::Skip => "#38bdf8",
            TestStatus::Timeout => "#F77600",
            TestStatus::Warn => "#ffdf20",
            TestStatus::Crash => "#e7000b",
        }
    }
}

fn percentage(count: usize, total: f32) -> f32 {
    (count as f32 * 100.0) / total
}

#[component]
fn LandingPlaceholder() -> Element {
    let stats_cards = TestStatus::iter().map(|s| {
        rsx! {
            StatCardPlaceholder {
                name: s.to_string(),
                color: s.color().to_string(),
                count: 0,
                stat: 0.0_f32,
            }
        }
    });

    let statuses = TestStatus::iter().enumerate().map(|(i, s)| {
        rsx! {
            SelectOption::<Option<TestStatus>> { index: i, value: s, text_value: "{s}",
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
                    p { class: "text-xs", "Total: 0 tests" }
                }
                div { class: "border-1 border-[#22c55e] bg-[#22c55e]/15 text-slate-400 w-fit rounded-3xl py-1 px-2 flex flex-row space-x-1 items-center",
                    div { class: "bg-[#22c55e] rounded-full size-3" }
                    p { class: "text-xs", "Filtered: 0 tests" }
                }
            }
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4",
                {stats_cards}
            }
            div { class: "mx-auto size-[200px]",
                Skeleton { class: "skeleton size-[200px] !rounded-full" }
            }
            div { class: "mt-12 w-full flex flex-col md:flex-row justify-between text-gray-400 text-sm gap-4 items-center",
                Skeleton { class: "hidden lg:block skeleton w-full h-9" }
                p { class: "my-auto w-fit text-nowrap", "Page 1 of 1" }
                div { class: "hidden lg:block",
                    PaginationPlaceholder { small: false }
                }
                div { class: "block lg:hidden",
                    PaginationPlaceholder { small: true }
                }
            }
            Skeleton { class: "block lg:hidden skeleton w-full h-9" }
            div { class: "w-full bg-gray-900 overflow-hidden border-1 border-slate-700 rounded-lg text-gray-400",
                table { class: "w-full border-collapse border-spacing-0",
                    tr {
                        class: "border-b-1 border-slate-700",
                        style: "background: radial-gradient(circle at top, rgba(56, 189, 248, 0.1), rgba(15, 23, 42, 1));",
                        th { class: "text-left uppercase bold whitespace-nowrap py-2 px-3",
                            "Test name"
                        }
                        th { class: "text-left uppercase bold whitespace-nowrap py-2 px-3",
                            "Duration (H:M:S.MS)"
                        }
                        th { class: "uppercase bold whitespace-nowrap py-2 px-3",
                            Select::<Option<TestStatus>> { placeholder: "STATUS",
                                SelectTrigger {
                                    class: "select-trigger mx-auto w-fit !bg-transparent !shadow-none !text-gray-400 cursor-pointer uppercase",
                                    aria_label: "Select Trigger",
                                    SelectValue { class: "!bg-transparent !shadow-none !text-gray-400" }
                                }
                                SelectList { aria_label: "Select status",
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
                    for _ in 0..12 {
                        tr { class: "text-sm hover:bg-[#38bef7]/5",
                            td { class: "py-2 px-3 w-full",
                                Skeleton { class: "skeleton w-[60%] h-6" }
                            }
                            td { class: "py-2",
                                Skeleton { class: "skeleton w-24 h-6 mx-auto" }
                            }
                            td { class: "py-2",
                                Skeleton { class: "skeleton w-16 h-9 !rounded-3xl mx-auto" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Landing() -> Element {
    let toast = use_toast();

    let resource = use_resource(move || {
        toast.info(
            "Loading...".to_string(),
            ToastOptions::new()
                .description("Loading CTS results")
                .duration(Duration::from_secs(12)),
        );
        get_results()
    })
    .load_with(rsx! {
        LandingPlaceholder {}
    })?;
    let mut result = use_signal(Vec::<StringRecord>::new);

    use_effect(move || match &*resource.read() {
        Ok(results) => {
            let mut reader = ReaderBuilder::new().from_reader(results.as_bytes());
            let records = match reader
                .into_records()
                .collect::<Result<Vec<StringRecord>, csv::Error>>()
            {
                Ok(res) => res,
                Err(e) => {
                    error!("Failed to parse results: {e}");
                    toast.error(
                        "Error".to_string(),
                        ToastOptions::new().description("Failed to parse CTS results"),
                    );
                    Vec::new()
                }
            };
            result.set(records);
            toast.success(
                "Success".to_string(),
                ToastOptions::new().description("Successfully loaded CTS results"),
            );
        }
        Err(e) => {
            error!("Failed to fetch results: {e}");
            toast.error(
                "Error".to_string(),
                ToastOptions::new().description("Failed to fetch CTS results"),
            );
        }
    });

    let global_stats = use_memo(move || {
        result.read().iter().fold(
            HashMap::from_iter(TestStatus::iter().map(|s| (s, 0))),
            |mut acc, record| {
                if let Ok(status) = TestStatus::from_str(&record[1]) {
                    *acc.entry(status).or_insert(0) += 1;
                }
                acc
            },
        )
    });

    let total = use_memo(move || {
        global_stats
            .read()
            .iter()
            .fold(0.0_f32, |acc, (_, v)| acc + *v as f32)
    });

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

    let mut current_page = use_memo(move || 0_usize);
    let mut search_input: Signal<Option<String>> = use_signal(|| None);
    let mut search_name: Signal<Option<String>> = use_signal(|| None);
    let mut filter: Signal<Option<TestStatus>> = use_signal(|| None);
    let filtered_count = use_memo(move || {
        let f = filter();
        let search = search_name();
        let rows = result.read();

        current_page.set(0_usize);

        let mut total = 0_usize;

        for r in rows.iter() {
            let Ok(status) = TestStatus::from_str(&r[1]) else {
                continue;
            };
            if let Some(wanted) = f {
                if status != wanted {
                    continue;
                }
            }

            let name = &r[0];
            if let Some(ref s) = search {
                if !name.contains(s) {
                    continue;
                }
            }

            total += 1;
        }

        total
    });
    let mut page_count = use_memo(move || filtered_count().max(PAGE_SIZE - 1) / PAGE_SIZE);
    let page = use_memo(move || {
        let _ = *filtered_count.read();

        let f = filter();
        let search = search_name();
        let shift = current_page() * PAGE_SIZE;

        let rows = result.read();

        let mut idx = 0_usize;
        let mut out = Vec::with_capacity(PAGE_SIZE);

        for r in rows.iter() {
            let name = &r[0];

            if let Some(ref s) = search {
                if !name.contains(s) {
                    continue;
                }
            }

            if let Some(wanted) = f {
                let Ok(status) = TestStatus::from_str(&r[1]) else {
                    continue;
                };
                if status != wanted {
                    continue;
                }
            }

            if idx >= shift && idx < shift + PAGE_SIZE {
                out.push(r.clone());
                if out.len() == PAGE_SIZE {
                    break;
                }
            }

            idx += 1;
        }

        out
    });

    let statuses = TestStatus::iter().enumerate().map(|(i, s)| {
        rsx! {
            SelectOption::<Option<TestStatus>> { index: i, value: s, text_value: "{s}",
                {format!("{} {s}", s.emoji())}
                SelectItemIndicator {}
            }
        }
    });

    let mut search_timeout: Signal<Option<TimeoutHandle>> = use_signal(|| None);
    let timeout = use_timeout(Duration::from_secs(1), move |()| {
        search_timeout.set(None);
        search_name.set(search_input());
    });

    let onsearch_input = move |event: FormEvent| {
        if event.value().is_empty() {
            search_input.set(None);
        } else {
            search_input.set(Some(event.value()));
        }
    };

    let onsearch_keyup = move |_| {
        if let Some(handle) = *search_timeout.read() {
            handle.cancel();
        }
        let handle = timeout.action(());
        search_timeout.set(Some(handle));
    };

    rsx! {
        div {
            class: "flex flex-col space-y-4 rounded-3xl p-4 pt-8 w-full h-fit shadow-xl shadow-slate-950",
            style: "background: linear-gradient(145deg, #020617 0, #02081f 60%, #020617 100%);",
            div { class: "flex flex-row space-x-4",
                div { class: "border-1 border-[#38bdf8] bg-[#38bdf8]/15 text-slate-400 w-fit rounded-3xl py-1 px-2 flex flex-row space-x-1 items-center",
                    div { class: "bg-[#38bdf8] rounded-full size-3" }
                    p { class: "text-xs", "Total: {total} tests" }
                }
                div { class: "border-1 border-[#22c55e] bg-[#22c55e]/15 text-slate-400 w-fit rounded-3xl py-1 px-2 flex flex-row space-x-1 items-center",
                    div { class: "bg-[#22c55e] rounded-full size-3" }
                    p { class: "text-xs", "Filtered: {filtered_count} tests" }
                }
            }
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4",
                {stats_cards}
            }
            div { class: "mx-auto size-[200px]",
                StatsPieChart { stats: global_stats, total }
            }
            div { class: "mt-12 w-full flex flex-col md:flex-row justify-between text-gray-400 text-sm gap-4 items-center",
                input {
                    class: "hidden lg:block w-full border-1 border-gray-700 px-3 py-1 rounded-lg text-sm",
                    style: "background: radial-gradient(circle at top, rgba(56, 189, 248, 0.1), rgba(15, 23, 42, 1));",
                    r#type: "search",
                    placeholder: "Search tests...",
                    oninput: onsearch_input,
                    onkeyup: onsearch_keyup,
                }
                p { class: "my-auto w-fit text-nowrap",
                    "Page {current_page() + 1} of {page_count() + 1}"
                }
                div { class: "hidden lg:block",
                    Pagination { current_page, page_count, small: false }
                }
                div { class: "block lg:hidden",
                    Pagination { current_page, page_count, small: true }
                }
            }
            input {
                class: "block lg:hidden w-full border-1 border-gray-700 px-3 py-1 rounded-lg text-sm",
                style: "background: radial-gradient(circle at top, rgba(56, 189, 248, 0.1), rgba(15, 23, 42, 1));",
                r#type: "search",
                placeholder: "Search tests...",
                oninput: onsearch_input,
                onkeyup: onsearch_keyup,
            }
            div { class: "w-full bg-gray-900 overflow-auto border-1 border-slate-700 rounded-lg text-gray-400",
                table { class: "w-full border-collapse border-spacing-0",
                    tr {
                        class: "border-b-1 border-slate-700",
                        style: "background: radial-gradient(circle at top, rgba(56, 189, 248, 0.1), rgba(15, 23, 42, 1));",
                        th { class: "text-left uppercase bold whitespace-nowrap py-2 px-3",
                            "Test name"
                        }
                        th { class: "text-left uppercase bold whitespace-nowrap py-2 px-3",
                            "Duration (H:M:S.MS)"
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
                                SelectList { aria_label: "Select status",
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
                            td { class: "py-2 px-3", "{&test[0]}" }
                            td { class: "py-2 px-3",
                                p { class: "mx-auto w-fit",
                                    if let Ok(duration) = test[2].parse::<f32>() {
                                        "{HMSDuration(Duration::from_secs_f32(duration))}"
                                    } else {
                                        "Invalid data"
                                    }
                                }
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
fn StatCardPlaceholder(name: String, color: String, count: usize, stat: f32) -> Element {
    rsx! {
        div { class: "rounded-2xl p-4 border-1 border-slate-800 shadow-xl shadow-[#02081f] w-full h-fit bg-[#090f21] flex flex-col space-y-2",
            div { class: "flex flex-row space-x-2 flex items-center",
                div {
                    class: "rounded-full size-4",
                    style: format!("background-color: {color};"),
                }
                h3 { class: "text-sm text-gray-300", "{name}" }
            }
            Skeleton { class: "skeleton w-32 h-7" }
            Skeleton { class: "skeleton w-24 h-4" }
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
                h3 { class: "text-sm text-gray-300", "{name}" }
            }
            h2 {
                class: "text-2xl font-bold",
                style: format!("color: {color};"),
                "{count}"
            }
            p { class: "text-xs text-gray-400", "{stat:.1}% of total" }
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
            style: format!(
                r#"
                                        background-color: {color}0F;
                                        color: {color};
                                        border-color: {color};
                                    "#,
            ),
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
        svg { class: "max-w-[200px] h-auto", view_box: "0 0 200 200", {paths} }
    }
}

#[component]
fn Pagination(current_page: Memo<usize>, page_count: ReadSignal<usize>, small: bool) -> Element {
    let range = if small { 1_usize } else { 2_usize };

    rsx! {
        div {
            class: "flex flex-row data-[is-small=true]:flow-col gap-2 max-w-screen overflow-x-auto",
            "data-is-small": small,
            div { class: "flex flex-row gap-2",
                button {
                    class: "pagination-button",
                    disabled: current_page() == 0,
                    onclick: move |_| current_page.set(0_usize),
                    if small {
                        "<<"
                    } else {
                        "First"
                    }
                }
                if !small {
                    button {
                        class: "pagination-button",
                        disabled: current_page() == 0,
                        onclick: move |_| current_page -= 1,
                        "Prev"
                    }
                }
            }

            div { class: "flex flex-row gap-2",
                if current_page() > range {
                    p { "..." }
                }

                for i in ((current_page() as i32 - range as i32).max(0)
                    as usize)..=(current_page() + range).min(page_count())
                {
                    button {
                        class: "pagination-button",
                        "data-active": i == current_page(),
                        onclick: move |_| current_page.set(i),
                        "{i + 1}"
                    }
                }

                if current_page() < (page_count() as i32 - range as i32).max(0) as usize {
                    p { "..." }
                }
            }

            div { class: "flex flex-row gap-2",
                if !small {
                    button {
                        class: "pagination-button",
                        disabled: current_page() >= page_count(),
                        onclick: move |_| current_page += 1,
                        "Next"
                    }
                }
                button {
                    class: "pagination-button",
                    disabled: current_page() >= page_count(),
                    onclick: move |_| current_page.set(page_count()),
                    if small {
                        ">>"
                    } else {
                        "Last"
                    }
                }
            }
        }
    }
}

#[component]
fn PaginationPlaceholder(small: bool) -> Element {
    let range = if small { 1_usize } else { 2_usize };

    rsx! {
        div {
            class: "flex flex-row data-[is-small=true]:flow-col gap-2 max-w-screen overflow-x-auto",
            "data-is-small": small,
            div { class: "flex flex-row gap-2",
                button { class: "pagination-button", disabled: true,
                    if small {
                        "<<"
                    } else {
                        "First"
                    }
                }
                if !small {
                    button { class: "pagination-button", disabled: true, "Prev" }
                }
            }

            div { class: "flex flex-row gap-2",
                for i in 1..(if small { 3 } else { 4 }) {
                    button { class: "pagination-button", disabled: true, "{i}" }
                }

                p { "..." }
            }

            div { class: "flex flex-row gap-2",
                if !small {
                    button { class: "pagination-button", disabled: true, "Next" }
                }
                button { class: "pagination-button", disabled: true,
                    if small {
                        ">>"
                    } else {
                        "Last"
                    }
                }
            }
        }
    }
}

async fn get_results() -> Result<String> {
    use async_zip::base::read::mem::ZipFileReader;
    let archive = reqwest::get(format!("{}/assets/results.zip", std::env!("URL")))
        .await?
        .bytes()
        .await?
        .to_vec();
    let zip = ZipFileReader::new(archive).await?;
    let mut string = String::new();
    let mut reader = zip.reader_with_entry(0).await?;
    reader.read_to_string_checked(&mut string).await?;
    Ok(string)
}
