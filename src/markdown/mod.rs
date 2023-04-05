use pulldown_cmark::{Event, Options, Parser, Tag};

mod html;

pub fn render(input: &str) -> String {
    let mut parser = Parser::new_ext(input, Options::all());
    let mut out = String::new();

    let mut footnotes = Vec::new();

    let mut main_events = Vec::new();

    // let mut footnote_counter = 1;
    // let mut footnote_names = HashMap::new();

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::FootnoteDefinition(name)) => {
                footnotes.push((name, collect_footnote(&mut parser)));
            }
            // Event::FootnoteReference(name) => {
            //     let id = footnote_counter;
            //     footnote_counter += 1;

            //     main_events.push(Event::FootnoteReference(pulldown_cmark::CowStr::Boxed(
            //         format!("fn:{id}").into_boxed_str(),
            //     )));

            //     assert_eq!(footnote_names.insert(name, id), None);
            // }
            _ => {
                main_events.push(event);
            }
        }
    }

    html::push_html(&mut out, main_events.into_iter());
    out
}

fn collect_footnote<'a>(p: &mut Parser<'a, '_>) -> Vec<Event<'a>> {
    let mut depth = 1;
    let mut events = Vec::new();
    while let Some(e) = p.next() {
        match e {
            Event::Start(_) => depth += 1,
            Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    return events;
                }
            }
            _ => {}
        };
        events.push(e);
    }

    unreachable!("footnote should be terminated by balanced End")
}
