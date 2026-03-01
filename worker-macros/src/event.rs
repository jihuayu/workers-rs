use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Error, Ident, ItemFn, ReturnType,
};

#[derive(Clone, Copy)]
enum HandlerType {
    Fetch,
    Scheduled,
    Email,
    Tail,
    Start,
    #[cfg(feature = "queue")]
    Queue,
}

pub fn expand_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs: Punctuated<Ident, Comma> =
        parse_macro_input!(attr with Punctuated::parse_terminated);

    use HandlerType::*;

    let mut handler_type = None;
    let mut respond_with_errors = false;

    for attr in attrs {
        match attr.to_string().as_str() {
            "fetch" => handler_type = Some(Fetch),
            "scheduled" => handler_type = Some(Scheduled),
            "email" => handler_type = Some(Email),
            "tail" => handler_type = Some(Tail),
            "start" => handler_type = Some(Start),
            #[cfg(feature = "queue")]
            "queue" => handler_type = Some(Queue),
            "respond_with_errors" => {
                respond_with_errors = true;
            }
            _ => panic!("Invalid attribute: {attr}"),
        }
    }
    let handler_type = handler_type.expect(
        "must have either 'fetch', 'scheduled', 'email', 'tail', 'queue' or 'start' attribute, e.g. #[event(fetch)]",
    );

    // create new var using syn item of the attributed fn
    let mut input_fn = parse_macro_input!(item as ItemFn);

    if let Err(err) = validate_signature(&input_fn, handler_type, respond_with_errors) {
        return err.to_compile_error().into();
    }

    match handler_type {
        Fetch => {
            // TODO: validate the inputs / signature
            // save original fn name for re-use in the wrapper fn
            let input_fn_ident = Ident::new(
                &(input_fn.sig.ident.to_string() + "_fetch_glue"),
                input_fn.sig.ident.span(),
            );
            let wrapper_fn_ident = Ident::new("fetch", input_fn.sig.ident.span());
            // rename the original attributed fn
            input_fn.sig.ident = input_fn_ident.clone();

            let error_handling = if respond_with_errors {
                quote! { ::worker::Response::error(e.to_string(), 500).unwrap().into() }
            } else {
                quote! { ::worker::Response::error("INTERNAL SERVER ERROR", 500).unwrap().into() }
            };

            // create a new "main" function that takes the worker_sys::Request, and calls the
            // original attributed function, passing in a converted worker::Request.
            // We use a synchronous wrapper that returns a Promise via future_to_promise
            // with AssertUnwindSafe to support panic=unwind.
            let wrapper_fn = quote! {
                pub fn #wrapper_fn_ident(
                    req: ::worker::worker_sys::web_sys::Request,
                    env: ::worker::Env,
                    ctx: ::worker::worker_sys::Context
                ) -> ::worker::js_sys::Promise {
                    ::worker::wasm_bindgen_futures::future_to_promise(::std::panic::AssertUnwindSafe(async move {
                        let ctx = worker::Context::new(ctx);
                        let response: ::worker::worker_sys::web_sys::Response = match ::worker::FromRequest::from_raw(req) {
                            Ok(req) => {
                                let result = #input_fn_ident(req, env, ctx).await;
                                // get the worker::Result<worker::Response> by calling the original fn
                                match result {
                                    Ok(raw_res) => {
                                        match ::worker::IntoResponse::into_raw(raw_res) {
                                            Ok(res) => res,
                                            Err(err) => {
                                                let e: Box<dyn std::error::Error> = err.into();
                                                ::worker::console_error!("Error converting response: {}", &e);
                                                #error_handling
                                            }
                                        }
                                    },
                                    Err(err) => {
                                        let e: Box<dyn std::error::Error> = err.into();
                                        ::worker::console_error!("{}", &e);
                                        #error_handling
                                    }
                                }
                            },
                            Err(err) => {
                                let e: Box<dyn std::error::Error> = err.into();
                                ::worker::console_error!("Error converting request: {}", &e);
                                #error_handling
                            }
                        };
                        Ok(::worker::wasm_bindgen::JsValue::from(response))
                    }))
                }
            };
            let wasm_bindgen_code =
                wasm_bindgen_macro_support::expand(TokenStream::new().into(), wrapper_fn)
                    .expect("wasm_bindgen macro failed to expand");

            let output = quote! {
                #input_fn

                mod _worker_fetch {
                    use ::worker::{wasm_bindgen, wasm_bindgen_futures};
                    use super::#input_fn_ident;
                    #wasm_bindgen_code
                }
            };

            TokenStream::from(output)
        }
        Scheduled => {
            // save original fn name for re-use in the wrapper fn
            let input_fn_ident = Ident::new(
                &(input_fn.sig.ident.to_string() + "_scheduled_glue"),
                input_fn.sig.ident.span(),
            );
            let wrapper_fn_ident = Ident::new("scheduled", input_fn.sig.ident.span());
            // rename the original attributed fn
            input_fn.sig.ident = input_fn_ident.clone();

            // Use a synchronous wrapper that returns a Promise via future_to_promise
            // with AssertUnwindSafe to support panic=unwind.
            let wrapper_fn = quote! {
                pub fn #wrapper_fn_ident(event: ::worker::worker_sys::ScheduledEvent, env: ::worker::Env, ctx: ::worker::worker_sys::ScheduleContext) -> ::worker::js_sys::Promise {
                    ::worker::wasm_bindgen_futures::future_to_promise(::std::panic::AssertUnwindSafe(async move {
                        // call the original fn
                        #input_fn_ident(::worker::ScheduledEvent::from(event), env, ::worker::ScheduleContext::from(ctx)).await;
                        Ok(::worker::wasm_bindgen::JsValue::UNDEFINED)
                    }))
                }
            };
            let wasm_bindgen_code =
                wasm_bindgen_macro_support::expand(TokenStream::new().into(), wrapper_fn)
                    .expect("wasm_bindgen macro failed to expand");

            let output = quote! {
                #input_fn

                mod _worker_scheduled {
                    use ::worker::{wasm_bindgen, wasm_bindgen_futures};
                    use super::#input_fn_ident;
                    #wasm_bindgen_code
                }
            };

            TokenStream::from(output)
        }
        Email => {
            let input_fn_ident = Ident::new(
                &(input_fn.sig.ident.to_string() + "_email_glue"),
                input_fn.sig.ident.span(),
            );
            let wrapper_fn_ident = Ident::new("email", input_fn.sig.ident.span());
            input_fn.sig.ident = input_fn_ident.clone();

            let wrapper_fn = quote! {
                pub fn #wrapper_fn_ident(event: ::worker::worker_sys::EmailMessage, env: ::worker::Env, ctx: ::worker::worker_sys::Context) -> ::worker::js_sys::Promise {
                    ::worker::wasm_bindgen_futures::future_to_promise(::std::panic::AssertUnwindSafe(async move {
                        let ctx = worker::Context::new(ctx);
                        match #input_fn_ident(::worker::EmailMessage::from(event), env, ctx).await {
                            Ok(()) => {},
                            Err(e) => {
                                ::worker::console_log!("{}", &e);
                                panic!("{}", e);
                            }
                        }
                        Ok(::worker::wasm_bindgen::JsValue::UNDEFINED)
                    }))
                }
            };
            let wasm_bindgen_code =
                wasm_bindgen_macro_support::expand(TokenStream::new().into(), wrapper_fn)
                    .expect("wasm_bindgen macro failed to expand");

            let output = quote! {
                #input_fn

                mod _worker_email {
                    use ::worker::{wasm_bindgen, wasm_bindgen_futures};
                    use super::#input_fn_ident;
                    #wasm_bindgen_code
                }
            };

            TokenStream::from(output)
        }
        Tail => {
            let input_fn_ident = Ident::new(
                &(input_fn.sig.ident.to_string() + "_tail_glue"),
                input_fn.sig.ident.span(),
            );
            let wrapper_fn_ident = Ident::new("tail", input_fn.sig.ident.span());
            input_fn.sig.ident = input_fn_ident.clone();

            let wrapper_fn = quote! {
                pub fn #wrapper_fn_ident(event: ::worker::worker_sys::TailEvent, env: ::worker::Env, ctx: ::worker::worker_sys::Context) -> ::worker::js_sys::Promise {
                    ::worker::wasm_bindgen_futures::future_to_promise(::std::panic::AssertUnwindSafe(async move {
                        let ctx = worker::Context::new(ctx);
                        match #input_fn_ident(::worker::TailEvent::from(event), env, ctx).await {
                            Ok(()) => {},
                            Err(e) => {
                                ::worker::console_log!("{}", &e);
                                panic!("{}", e);
                            }
                        }
                        Ok(::worker::wasm_bindgen::JsValue::UNDEFINED)
                    }))
                }
            };
            let wasm_bindgen_code =
                wasm_bindgen_macro_support::expand(TokenStream::new().into(), wrapper_fn)
                    .expect("wasm_bindgen macro failed to expand");

            let output = quote! {
                #input_fn

                mod _worker_tail {
                    use ::worker::{wasm_bindgen, wasm_bindgen_futures};
                    use super::#input_fn_ident;
                    #wasm_bindgen_code
                }
            };

            TokenStream::from(output)
        }
        #[cfg(feature = "queue")]
        Queue => {
            // save original fn name for re-use in the wrapper fn
            let input_fn_ident = Ident::new(
                &(input_fn.sig.ident.to_string() + "_queue_glue"),
                input_fn.sig.ident.span(),
            );
            let wrapper_fn_ident = Ident::new("queue", input_fn.sig.ident.span());
            // rename the original attributed fn
            input_fn.sig.ident = input_fn_ident.clone();

            // Use a synchronous wrapper that returns a Promise via future_to_promise
            // with AssertUnwindSafe to support panic=unwind.
            let wrapper_fn = quote! {
                pub fn #wrapper_fn_ident(event: ::worker::worker_sys::MessageBatch, env: ::worker::Env, ctx: ::worker::worker_sys::Context) -> ::worker::js_sys::Promise {
                    ::worker::wasm_bindgen_futures::future_to_promise(::std::panic::AssertUnwindSafe(async move {
                        // call the original fn
                        let ctx = worker::Context::new(ctx);
                        match #input_fn_ident(::worker::MessageBatch::from(event), env, ctx).await {
                            Ok(()) => {},
                            Err(e) => {
                                ::worker::console_log!("{}", &e);
                                panic!("{}", e);
                            }
                        }
                        Ok(::worker::wasm_bindgen::JsValue::UNDEFINED)
                    }))
                }
            };
            let wasm_bindgen_code =
                wasm_bindgen_macro_support::expand(TokenStream::new().into(), wrapper_fn)
                    .expect("wasm_bindgen macro failed to expand");

            let output = quote! {
                #input_fn

                mod _worker_queue {
                    use ::worker::{wasm_bindgen, wasm_bindgen_futures};
                    use super::#input_fn_ident;
                    #wasm_bindgen_code
                }
            };

            TokenStream::from(output)
        }
        Start => {
            // save original fn name for re-use in the wrapper fn
            let input_fn_ident = Ident::new(
                &(input_fn.sig.ident.to_string() + "_start_glue"),
                input_fn.sig.ident.span(),
            );
            let wrapper_fn_ident = Ident::new("start", input_fn.sig.ident.span());
            // rename the original attributed fn
            input_fn.sig.ident = input_fn_ident.clone();

            let wrapper_fn = quote! {
                pub fn #wrapper_fn_ident() {
                    // call the original fn
                    #input_fn_ident()
                }
            };
            let wasm_bindgen_code =
                wasm_bindgen_macro_support::expand(quote! { start }, wrapper_fn)
                    .expect("wasm_bindgen macro failed to expand");

            let output = quote! {
                #input_fn

                mod _worker_start {
                    use ::worker::{wasm_bindgen, wasm_bindgen_futures};
                    use super::#input_fn_ident;
                    #wasm_bindgen_code
                }
            };

            TokenStream::from(output)
        }
    }
}

fn validate_signature(
    input_fn: &ItemFn,
    handler_type: HandlerType,
    respond_with_errors: bool,
) -> syn::Result<()> {
    match handler_type {
        HandlerType::Fetch => {
            if !input_fn.sig.asyncness.is_some() {
                return Err(Error::new_spanned(
                    &input_fn.sig,
                    "#[event(fetch)] handler must be async",
                ));
            }
            if input_fn.sig.inputs.len() != 3 {
                return Err(Error::new_spanned(
                    &input_fn.sig.inputs,
                    "#[event(fetch)] handler must accept exactly 3 parameters: request, env, ctx",
                ));
            }
            validate_result_return(input_fn, "#[event(fetch)]")
        }
        HandlerType::Scheduled => {
            if !input_fn.sig.asyncness.is_some() {
                return Err(Error::new_spanned(
                    &input_fn.sig,
                    "#[event(scheduled)] handler must be async",
                ));
            }
            if input_fn.sig.inputs.len() != 3 {
                return Err(Error::new_spanned(
                    &input_fn.sig.inputs,
                    "#[event(scheduled)] handler must accept exactly 3 parameters: event, env, ctx",
                ));
            }
            validate_result_return(input_fn, "#[event(scheduled)]")
        }
        HandlerType::Email => {
            if !input_fn.sig.asyncness.is_some() {
                return Err(Error::new_spanned(
                    &input_fn.sig,
                    "#[event(email)] handler must be async",
                ));
            }
            if input_fn.sig.inputs.len() != 3 {
                return Err(Error::new_spanned(
                    &input_fn.sig.inputs,
                    "#[event(email)] handler must accept exactly 3 parameters: message, env, ctx",
                ));
            }
            validate_result_return(input_fn, "#[event(email)]")
        }
        HandlerType::Tail => {
            if !input_fn.sig.asyncness.is_some() {
                return Err(Error::new_spanned(
                    &input_fn.sig,
                    "#[event(tail)] handler must be async",
                ));
            }
            if input_fn.sig.inputs.len() != 3 {
                return Err(Error::new_spanned(
                    &input_fn.sig.inputs,
                    "#[event(tail)] handler must accept exactly 3 parameters: event, env, ctx",
                ));
            }
            validate_result_return(input_fn, "#[event(tail)]")
        }
        #[cfg(feature = "queue")]
        HandlerType::Queue => {
            if !input_fn.sig.asyncness.is_some() {
                return Err(Error::new_spanned(
                    &input_fn.sig,
                    "#[event(queue)] handler must be async",
                ));
            }
            if input_fn.sig.inputs.len() != 3 {
                return Err(Error::new_spanned(
                    &input_fn.sig.inputs,
                    "#[event(queue)] handler must accept exactly 3 parameters: batch, env, ctx",
                ));
            }
            validate_result_return(input_fn, "#[event(queue)]")
        }
        HandlerType::Start => {
            if input_fn.sig.asyncness.is_some() {
                return Err(Error::new_spanned(
                    &input_fn.sig,
                    "#[event(start)] handler must not be async",
                ));
            }
            if !input_fn.sig.inputs.is_empty() {
                return Err(Error::new_spanned(
                    &input_fn.sig.inputs,
                    "#[event(start)] handler must not accept any parameters",
                ));
            }
            if !matches!(input_fn.sig.output, ReturnType::Default) {
                return Err(Error::new_spanned(
                    &input_fn.sig.output,
                    "#[event(start)] handler must not return a value",
                ));
            }
            Ok(())
        }
    }?;

    if respond_with_errors && !matches!(handler_type, HandlerType::Fetch) {
        return Err(Error::new_spanned(
            &input_fn.sig,
            "respond_with_errors is only valid for #[event(fetch)] handlers",
        ));
    }

    Ok(())
}

fn validate_result_return(input_fn: &ItemFn, event_name: &str) -> syn::Result<()> {
    match &input_fn.sig.output {
        ReturnType::Type(_, ty) => {
            if quote!(#ty).to_string().contains("Result") {
                Ok(())
            } else {
                Err(Error::new_spanned(
                    ty,
                    format!("{event_name} handler must return Result<...>"),
                ))
            }
        }
        ReturnType::Default => Err(Error::new_spanned(
            &input_fn.sig,
            format!("{event_name} handler must return Result<...>"),
        )),
    }
}
