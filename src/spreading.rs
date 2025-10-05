//! This file is copied from https://github.com/leptos-rs/leptos/blob/main/examples/spread/src/lib.rs

use leptos::prelude::*;

/// Demonstrates how attributes and event handlers can be spread onto elements.
#[component]
pub fn SpreadingExample() -> impl IntoView {
    fn alert(msg: impl AsRef<str>) {
        let _ = window().alert_with_message(msg.as_ref());
    }

    // you can easily create sets of spreadable attributes by using the <{..} ___/> syntax
    // this is expanded to a tuple of attributes; it has no meaning on its own, but can be spread
    // onto an HTML element or component
    let attrs_only = view! { <{..} class="foo" /> };
    let event_handlers_only = view! {
        <{..} on:click=move |_| {
            alert("event_handlers_only clicked");
        } />
    };
    let combined = view! { <{..} class="bar" on:click=move |_| alert("combined clicked") /> };
    let partial_attrs = view! { <{..} id="snood" class="baz" data-foo="bar" /> };
    let partial_event_handlers =
        view! { <{..} on:click=move |_| alert("partial_event_handlers clicked") /> };
    let spread_onto_component =
        view! { <{..} aria-label="a component with attribute spreading" /> };

    /* with the correct imports, you can use a tuple/builder syntax as well
        let attrs_only = class("foo");
        let event_handlers_only = on(ev::click, move |_e: ev::MouseEvent| {
            alert("event_handlers_only clicked");
        });
        let combined = (
            class("bar"),
            on(ev::click, move |_e: ev::MouseEvent| {
                alert("combined clicked");
            }),
        );
        let partial_attrs = (id("snood"), class("baz"));
        let partial_event_handlers = on(ev::click, move |_e: ev::MouseEvent| {
            alert("partial_event_handlers clicked");
        });
    */

    view! {
        <p>
            "You can spread any valid attribute, including a tuple of attributes, with the {..attr} syntax"
        </p>
        <div {..attrs_only}>"<div {..attrs_only} />"</div>

        <div {..event_handlers_only}>"<div {..event_handlers_only} />"</div>

        <div {..combined}>"<div {..combined} />"</div>

        <div {..partial_attrs} {..partial_event_handlers}>
            "<div {..partial_attrs} {..partial_event_handlers} />"
        </div>

        <hr />

        // attributes that are spread onto a component will be applied to *all* elements returned as part of
        // the component's view. to apply attributes to a subset of the component, pass them via a component prop
        <ComponentThatTakesSpread
            // the class:, style:, prop:, on: syntaxes work just as they do on elements
            class:foo=true
            style:font-weight="bold"
            prop:cool=42
            on:click=move |_| alert("clicked ComponentThatTakesSpread")
            // props are passed as they usually are on components
            some_prop=13
            // to pass a plain HTML attribute, prefix it with attr:
            attr:id="foo"
            // or, if you want to include multiple attributes, rather than prefixing each with
            // attr:, you can separate them from component props with the spread {..}
            // everything after this is treated as an HTML attribute
            {..}
            title="ooh, a title!"
            {..spread_onto_component}
        />
    }
}

#[component]
pub fn ComponentThatTakesSpread(some_prop: i32) -> impl IntoView {
    leptos::logging::log!("some_prop = {some_prop}");
    view! {
        <button>"<ComponentThatTakesSpread/>"</button>
        <p>
            "Attributes applied to a component apply to all top-level elements returned by a component."
        </p>
    }
}
