use dioxus::prelude::*;

/// Default placeholder for loading state
#[component]
pub fn LoadingPlaceholder(message: String) -> Element {
    rsx! {
        div { class: "rounded-radius h-[50vh] w-1/2 bg-foreground/20 animate-pulse text-foreground flex justify-center items-center place-self-center mt-14 mx-auto",
            h3 { class: "h3", {message} }
        }
    }
}

#[derive(Default, Clone)]
struct SuspenseContextPlaceholder {
    element: Option<Element>,
}

/// Trait to create a suspense with a default loading placeholder
pub trait Loader<T: 'static> {
    fn load(
        &self,
        message: impl ToString,
    ) -> Result<MappedSignal<T, Signal<Option<T>> /* wtf Dioxus ??? */>, RenderError>;
    fn load_with(
        &self,
        element: Element,
    ) -> Result<MappedSignal<T, Signal<Option<T>>>, RenderError>;
}

impl<T> Loader<T> for Resource<T> {
    fn load(
        &self,
        message: impl ToString,
    ) -> Result<MappedSignal<T, Signal<Option<T>>>, RenderError> {
        let mut context = use_context::<Signal<SuspenseContextPlaceholder>>();
        context.write().element = Some(rsx! {
            LoadingPlaceholder { message: message.to_string() }
        });
        self.suspend()
    }

    fn load_with(
        &self,
        element: Element,
    ) -> Result<MappedSignal<T, Signal<Option<T>>>, RenderError> {
        let mut context = use_context::<Signal<SuspenseContextPlaceholder>>();
        context.write().element = Some(element);
        self.suspend()
    }
}

#[component]
pub fn Suspense(children: Element) -> Element {
    let context = use_context_provider(|| Signal::new(SuspenseContextPlaceholder::default()));

    rsx! {
        SuspenseBoundary { fallback: move |_| { context.read().clone().element.unwrap_or_else(|| rsx! { "Loading..." }) },
            {children}
        }
    }
}
